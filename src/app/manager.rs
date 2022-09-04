use crate::app::types::{Country, Countries, Output};
use crate::app::utils::{read_countries_from_file, write_countries_to_file};

pub fn add(filename: &str, country: &Country, countries: &Countries) -> Output {
    let mut countries = read_countries_from_file(filename, countries);
    countries.push(country.clone());

    match write_countries_to_file(filename, &countries) {
        Some(_) => Output::Add(country.clone()),
        None    => Output::Unwritable
    }
}

pub fn remove(filename: &str, country: &Country, countries: &Countries) -> Output {
    let countries = read_countries_from_file(filename, countries).into_iter().filter(|x| x != country).collect::<Countries>();

    match write_countries_to_file(filename, &countries) {
        Some(_) => Output::Remove(country.clone()),
        None    => Output::Unwritable
    }
}

pub fn list(filename: &str, countries: &Countries) -> Output {
    Output::List(read_countries_from_file(filename, countries))
}


#[cfg(test)]
mod tests {
    use std::fs::{remove_file};

    use super::*;
    use crate::app::mocks::get_mock_countries;

    pub const MOCK_FILENAME: &str = "been-there.manager.mock.json";

    #[test]
    fn it_can_add_and_remove_countries() {
        let (countries, france, spain, greece) = get_mock_countries();
        assert_eq!(list(MOCK_FILENAME, &countries), Output::List(vec![]));

        assert_eq!(add(MOCK_FILENAME, &france, &countries), Output::Add(france.clone()));
        let expected_countries = Output::List(vec![france.clone()]);
        assert_eq!(list(MOCK_FILENAME, &countries), expected_countries);

        assert_eq!(add(MOCK_FILENAME, &spain, &countries), Output::Add(spain.clone()));
        let expected_countries = Output::List(vec![france.clone(), spain.clone()]);
        assert_eq!(list(MOCK_FILENAME, &countries), expected_countries);

        assert_eq!(add(MOCK_FILENAME, &greece, &countries), Output::Add(greece.clone()));
        let expected_countries = Output::List(vec![france.clone(), spain.clone(), greece.clone()]);
        assert_eq!(list(MOCK_FILENAME, &countries), expected_countries);

        assert_eq!(remove(MOCK_FILENAME, &france, &countries), Output::Remove(france.clone()));
        let expected_countries = Output::List(vec![spain.clone(), greece.clone()]);
        assert_eq!(list(MOCK_FILENAME, &countries), expected_countries);

        remove_file(MOCK_FILENAME).ok();
    }
}