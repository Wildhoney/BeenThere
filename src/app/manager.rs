use crate::app::types::{Country, Countries, Output};
use crate::app::utils::{read_countries_from_file, write_countries_to_file};

use super::utils::FILENAME;

pub fn add(country: Country, countries: Countries) -> Output {
    let mut countries = read_countries_from_file(FILENAME, countries);
    countries.push(country.clone());

    match write_countries_to_file(FILENAME, countries) {
        Some(_) => Output::Add(country),
        None    => Output::Unwritable
    }
}

pub fn remove(country: Country, countries: Countries) -> Output {
    let countries = read_countries_from_file(FILENAME, countries).into_iter().filter(|x| *x != country).collect::<Countries>();

    match write_countries_to_file(FILENAME, countries) {
        Some(_) => Output::Remove(country),
        None    => Output::Unwritable
    }
}

pub fn list(countries: Countries) -> Output {
    Output::List(read_countries_from_file(FILENAME, countries))
}
