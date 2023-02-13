use crate::app::types::{Countries, Country, Output};
use crate::app::utils::{read_countries_from_file, write_countries_to_file};

use super::types::{Info, List};
use super::utils::has_visited_country;

pub fn add(filename: &str, country: &Country, countries: &Countries) -> Output {
    let mut countries = read_countries_from_file(filename, countries);
    countries.push(country.clone());

    match write_countries_to_file(filename, &countries) {
        Some(_) => Output::Add(country.clone()),
        None => Output::Unwritable,
    }
}

pub fn remove(filename: &str, country: &Country, countries: &Countries) -> Output {
    let countries = read_countries_from_file(filename, countries)
        .into_iter()
        .filter(|x| x != country)
        .collect::<Countries>();

    match write_countries_to_file(filename, &countries) {
        Some(_) => Output::Remove(country.clone()),
        None => Output::Unwritable,
    }
}

pub fn list(filename: &str, countries: &Countries) -> Output {
    Output::List(List {
        visited_countries: read_countries_from_file(filename, &countries),
        total_countries: countries.len(),
    })
}

pub fn info(filename: &str, country: &Country, countries: &Countries) -> Output {
    Output::Info(Info {
        country: country.to_owned(),
        visited_countries: countries.to_owned(),
        has_visited: has_visited_country(filename, country, countries),
    })
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;

    use crate::app::mocks::{get_mocked_countries, MockedCountries};

    use super::*;

    pub const MOCK_FILENAME: &str = "been-there.manager.mock.json";

    #[test]
    fn it_can_add_and_remove_countries() {
        let MockedCountries {
            all,
            france,
            spain,
            greece,
        } = get_mocked_countries();

        assert_eq!(
            list(MOCK_FILENAME, &all),
            Output::List(List {
                visited_countries: vec![],
                total_countries: 3
            })
        );

        assert_eq!(
            add(MOCK_FILENAME, &france, &all),
            Output::Add(france.clone())
        );
        let expected_countries = Output::List(List {
            visited_countries: vec![france.clone()],
            total_countries: 3,
        });
        assert_eq!(list(MOCK_FILENAME, &all), expected_countries);

        assert_eq!(add(MOCK_FILENAME, &spain, &all), Output::Add(spain.clone()));
        let expected_countries = Output::List(List {
            visited_countries: vec![france.clone(), spain.clone()],
            total_countries: 3,
        });
        assert_eq!(list(MOCK_FILENAME, &all), expected_countries);

        assert_eq!(
            add(MOCK_FILENAME, &greece, &all),
            Output::Add(greece.clone())
        );
        let expected_countries = Output::List(List {
            visited_countries: vec![france.clone(), spain.clone(), greece.clone()],
            total_countries: 3,
        });
        assert_eq!(list(MOCK_FILENAME, &all), expected_countries);

        assert_eq!(
            remove(MOCK_FILENAME, &france, &all),
            Output::Remove(france.clone())
        );
        let expected_countries = Output::List(List {
            visited_countries: vec![spain.clone(), greece.clone()],
            total_countries: 3,
        });
        assert_eq!(list(MOCK_FILENAME, &all), expected_countries);

        remove_file(MOCK_FILENAME).ok();
    }
}
