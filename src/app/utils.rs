use itertools::Itertools;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::{fs, vec};

use crate::app::types::{Continents, Countries, Country};

const COUNTRY_URL: &str = "https://restcountries.com/v3.1/all";

pub async fn get_countries_from_remote() -> Option<Countries> {
    match reqwest::get(COUNTRY_URL).await {
        Ok(response) => response.json::<Countries>().await.ok(),
        Err(_) => None,
    }
}

pub fn get_country_by_name(country: &str, countries: &Countries) -> Option<Country> {
    countries.clone().into_iter().find(|x| {
        let name = country.to_lowercase();
        let is_name_matched = x.name.common.to_lowercase() == name;
        let mut alt_spellings = x
            .alt_spellings
            .clone()
            .into_iter()
            .map(|name| name.to_lowercase());
        let is_alt_spellings_matched = alt_spellings.any(|x| x == name);

        is_name_matched || is_alt_spellings_matched
    })
}

pub fn read_countries_from_file(filename: &str, countries: &Countries) -> Countries {
    let buffer = fs::read_to_string(filename).unwrap_or_else(|_| "[]".to_string());
    let names = serde_json::from_str::<Vec<String>>(&buffer).unwrap_or_else(|_| vec![]);

    countries
        .clone()
        .into_iter()
        .filter_map(|country| names.contains(&country.name.common).then_some(country))
        .collect::<Countries>()
}

pub fn write_countries_to_file(filename: &str, countries: &Countries) -> Option<()> {
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open(filename);
    let names = countries
        .iter()
        .map(|country| country.name.common.clone())
        .unique()
        .collect::<Vec<String>>();

    match file {
        Ok(mut file) => file
            .write_all(serde_json::to_string_pretty(&names).unwrap().as_bytes())
            .ok(),
        Err(_) => None,
    }
}

pub fn has_visited_country(filename: &str, country: &Country, countries: &Countries) -> bool {
    let countries = read_countries_from_file(filename, &countries);
    countries.contains(country)
}

pub fn get_countries_by_people(countries: &Countries) -> (Country, Country) {
    let mut countries = countries.clone();
    countries.sort_by(|a, b| a.population.cmp(&b.population));
    (
        countries.first().unwrap().to_owned(),
        countries.last().unwrap().to_owned(),
    )
}

pub fn get_countries_by_land(countries: &Countries) -> (Country, Country) {
    let mut countries = countries.clone();
    countries.sort_by(|a, b| (a.area as usize).cmp(&(b.area as usize)));
    (
        countries.first().unwrap().to_owned(),
        countries.last().unwrap().to_owned(),
    )
}

pub fn get_visited_continents(countries: &Countries) -> Continents {
    let continents = countries
        .iter()
        .flat_map(|country| country.continents.clone());
    let mut continents = continents.fold(vec![], |mut continents, continent| {
        let current = continents
            .clone()
            .into_iter()
            .find(|(x, _)| *x == continent);

        match current {
            Some((continent, count)) => {
                let mut continents = continents
                    .into_iter()
                    .filter(|(x, _)| *x != continent)
                    .collect::<Vec<_>>();
                continents.push((continent, count + 1));
                continents
            }
            None => {
                continents.push((continent, 1));
                continents
            }
        }
    });

    continents.sort_by(|(_, a), (_, b)| b.cmp(a));
    continents
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;

    use super::*;
    use crate::app::{manager::add, mocks::get_mock_countries};

    pub const MOCK_FILENAME: &str = "been-there.utils.mock.json";

    #[test]
    fn it_can_get_countries_by_name() {
        let (countries, _, spain, _) = get_mock_countries();
        assert_eq!(
            get_country_by_name(&"spain".to_string(), &countries),
            Some(spain)
        );
    }

    #[test]
    fn it_can_get_countries_by_alt_spellings() {
        let (countries, france, _, _) = get_mock_countries();
        assert_eq!(
            get_country_by_name(&"fr".to_string(), &countries),
            Some(france)
        );
    }

    #[test]
    fn it_can_get_min_max_people_from_countries() {
        let (countries, france, _, greece) = get_mock_countries();
        let (fewest_people, most_people) = get_countries_by_people(&countries);
        assert_eq!(most_people, greece);
        assert_eq!(fewest_people, france);
    }

    #[test]
    fn it_can_get_min_max_land_from_countries() {
        let (countries, _, spain, greece) = get_mock_countries();
        let (least_land, most_land) = get_countries_by_land(&countries);
        assert_eq!(most_land, spain);
        assert_eq!(least_land, greece);
    }

    #[test]
    fn it_should_read_and_write_from_file() {
        let (countries, _, _, _) = get_mock_countries();
        assert_eq!(write_countries_to_file(MOCK_FILENAME, &countries), Some(()));
        assert_eq!(
            read_countries_from_file(MOCK_FILENAME, &countries),
            countries.clone()
        );

        remove_file(MOCK_FILENAME).ok();
    }

    #[test]
    fn it_can_get_sorted_continents_from_countries() {
        let (countries, _, _, _) = get_mock_countries();
        let continents = get_visited_continents(&countries);
        assert_eq!(continents, vec![("Europe".to_string(), 3)]);
    }

    #[test]
    fn it_should_know_if_weve_visited_a_country() {
        let (countries, france, _, _) = get_mock_countries();
        assert_eq!(
            has_visited_country(MOCK_FILENAME, &france, &countries),
            false
        );
        add(MOCK_FILENAME, &france, &countries);
        assert_eq!(
            has_visited_country(MOCK_FILENAME, &france, &countries),
            true
        );

        remove_file(MOCK_FILENAME).ok();
    }
}
