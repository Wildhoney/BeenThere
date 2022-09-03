use std::fs;
use std::io::prelude::*;
use std::fs::OpenOptions;
use itertools::Itertools;

use crate::app::types::{Countries, Country};

pub const FILENAME: &str = "been-there.json";

const COUNTRY_URL: &str = "https://restcountries.com/v3.1/all";

pub async fn get_countries_from_remote() -> Option<Countries> {
    match reqwest::get(COUNTRY_URL).await {
        Ok(response) => response.json::<Countries>().await.ok(),
        Err(_)       => None
    }
}

pub fn get_country_by_name(country: &String, countries: Countries) -> Option<Country> {
    countries.into_iter().find(|x| {
        let name = country.to_lowercase();
        let is_name_matched = x.name.common.to_lowercase() == name;
        let alt_spellings = x.alt_spellings.clone().into_iter().map(|name| name.to_lowercase()).collect::<Vec<_>>();
        let is_alt_spelling_matched = alt_spellings.contains(&name);

        is_name_matched || is_alt_spelling_matched
    })
}

pub fn read_countries_from_file(countries: Countries) -> Countries {
    let mut buffer = fs::read_to_string(FILENAME).unwrap_or_else(|_| "[]".to_string());
    let names = serde_json::from_str::<Vec<String>>(&mut buffer).unwrap_or_else(|_| vec![]);
    countries.into_iter().filter_map(|country| names.contains(&country.name.common).then_some(country)).collect::<Countries>()
}

pub fn write_countries_to_file(countries: Countries) -> Option<()> {
    let file = OpenOptions::new().write(true).read(true).create(true).truncate(true).open(FILENAME);
    let names = countries.into_iter().map(|country| country.name.common).unique().collect::<Vec<String>>();

    match file {
        Ok(mut file) => file.write_all(serde_json::to_string_pretty(&names).unwrap().as_bytes()).ok(),
        Err(_)       => None
    }
}

pub fn get_countries_by_people(countries: &Countries) -> (Country, Country) {
    let mut countries = countries.clone();
    countries.sort_by(|a, b| b.population.cmp(&a.population));
    (countries.first().unwrap().to_owned(), countries.last().unwrap().to_owned())
}

pub fn get_countries_by_land(countries: &Countries) -> (Country, Country) {
    let mut countries = countries.clone();
    countries.sort_by(|a, b| (b.area as usize).cmp(&(a.area as usize)));
    (countries.first().unwrap().to_owned(), countries.last().unwrap().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::mocks::get_mock_countries;

    #[test]
    fn it_can_get_countries_by_name() {
        let (countries, _, spain, _) = get_mock_countries();
        assert_eq!(get_country_by_name(&"spain".to_string(), countries), Some(spain));
    }

    #[test]
    fn it_can_get_countries_by_alt_spellings() {
        let (countries, france, _, _) = get_mock_countries();
        assert_eq!(get_country_by_name(&"fr".to_string(), countries), Some(france));
    }

    #[test]
    fn it_can_get_min_max_people_from_countries() {
        let (countries, france, _, greece) = get_mock_countries();
        let (most_people, fewest_people)   = get_countries_by_people(&countries);
        assert_eq!(most_people, greece);
        assert_eq!(fewest_people, france);
    }

    #[test]
    fn it_can_get_min_max_land_from_countries() {
        let (countries, _, spain, greece) = get_mock_countries();
        let (most_land, least_land)       = get_countries_by_land(&countries);
        assert_eq!(most_land, spain);
        assert_eq!(least_land, greece);
    }
}