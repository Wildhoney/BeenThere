use std::io::prelude::*;
use std::fs::OpenOptions;

use crate::app::types::{Country, Output};
use crate::app::utils::read_countries_from_file;

const FILENAME: &str = "been-there.json";

pub fn add(country: Country) -> Result<Output, String> {
    let mut countries = read_countries_from_file(FILENAME);
    let file = OpenOptions::new().write(true).read(true).create(true).truncate(true).open(FILENAME);

    match file {
        Ok(mut file) => {
            countries.push(country.clone());

            match file.write_all(serde_json::to_string(&countries).unwrap().as_bytes()) {
                Ok(_)  => Ok(Output::Add(country)),
                Err(_) => Err(format!("Cannot add {:?}", country.name))
            }
        },
        Err(_)  => Err(format!("Cannot open file: {}", FILENAME))
    }
}

pub fn list() -> Result<Output, String> {
    Ok(Output::List(read_countries_from_file(FILENAME)))
}