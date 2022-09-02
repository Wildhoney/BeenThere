use std::fs;
use std::io::prelude::*;
use std::fs::OpenOptions;
use itertools::Itertools;

use crate::app::types::{Countries, Country};

const FILENAME: &str = "been-there.json";

pub fn get_country(country: &String, countries: Countries) -> Result<Country, String> {
    match countries.into_iter().find(|x| {
        let name = country.to_lowercase();
        let is_name_matched = x.name.common.to_lowercase() == name;
        let is_cioc_matched = *x.cioc.as_ref().unwrap_or(&"".to_string()).to_lowercase() == name;
        return is_name_matched || is_cioc_matched;
    }) {
        Some(country) => Ok(country),
        _             => Err(format!("We're unable to find the country '{}'", country))
    }

}

pub fn read_countries_from_file(countries: Countries) -> Countries {
    let mut buffer = fs::read_to_string(FILENAME).unwrap_or_else(|_| "[]".to_string());
    let names = serde_json::from_str::<Vec<String>>(&mut buffer).unwrap_or_else(|_| vec![]);

    countries.into_iter().filter_map(|country| match names.contains(&country.name.common) {
        true  => Some(country),
        false => None
    }).collect::<Countries>()
}

pub fn write_countries_to_file(countries: Countries) -> Option<()> {
    let file = OpenOptions::new().write(true).read(true).create(true).truncate(true).open(FILENAME);
    let names = countries.into_iter().map(|country| country.name.common).unique().collect::<Vec<String>>();

    match file {
        Ok(mut file) => {
            let result = file.write_all(serde_json::to_string_pretty(&names).unwrap().as_bytes());

            match result {
                Ok(_)  => Some(()),
                Err(_) => None
            }
        },
        Err(_)   => None
    }
}
