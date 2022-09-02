use crate::app::types::{Country, Countries, Output};
use crate::app::utils::{read_countries_from_file, write_countries_to_file};

pub fn add(country: Country, countries: Countries) -> Result<Output, String> {
    let mut countries = read_countries_from_file(countries);
    countries.push(country.clone());
    
    match write_countries_to_file(countries) {
        Some(_) => Ok(Output::Add(country)),
        None    => Err(format!("Cannot add {:?}", country.name))
    }
}

pub fn remove(country: Country, countries: Countries) -> Result<Output, String> {
    let countries = read_countries_from_file(countries).into_iter().filter(|x| *x != country).collect::<Countries>();

    match write_countries_to_file(countries) {
        Some(_) => Ok(Output::Remove(country)),
        None    => Err(format!("Cannot remove {:?}", country.name))
    }
}

pub fn list(countries: Countries) -> Result<Output, String> {
    Ok(Output::List(read_countries_from_file(countries)))
}
