use crate::app::types::{Country, Countries, Output};
use crate::app::utils::{read_countries_from_file, write_countries_to_file};

pub fn add(country: Country, countries: Countries) -> Output {
    let mut countries = read_countries_from_file(countries);
    countries.push(country.clone());

    match write_countries_to_file(countries) {
        Some(_) => Output::Add(country),
        None    => Output::Unwritable
    }
}

pub fn remove(country: Country, countries: Countries) -> Output {
    let countries = read_countries_from_file(countries).into_iter().filter(|x| *x != country).collect::<Countries>();

    match write_countries_to_file(countries) {
        Some(_) => Output::Remove(country),
        None    => Output::Unwritable
    }
}

pub fn list(countries: Countries) -> Output {
    Output::List(read_countries_from_file(countries))
}
