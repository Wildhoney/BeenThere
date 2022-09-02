use std::fs;

use crate::app::types::{Countries, Country};

pub fn get_country(country: &String, countries: Countries) -> Result<Country, String> {
    match countries.into_iter().find(|x| &x.name.common == country) {
        Some(country) => Ok(country),
        _             => Err(format!("We're unable to find the country '{}'", country))
    }

}

pub fn read_countries_from_file(filename: &str) -> Countries {
    let mut buffer = fs::read_to_string(filename).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str::<Vec<Country>>(&mut buffer).unwrap_or_else(|_| vec![])
}
