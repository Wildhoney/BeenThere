use std::fs;
use std::io::prelude::*;
use std::fs::OpenOptions;

use crate::app::types::{Country, Output};

const FILENAME: &str = "been-there.json";

pub fn add(country: Country) -> Result<Output, String> {
    let mut buffer = fs::read_to_string(FILENAME).unwrap_or_else(|_| "[]".to_string());
    let file = OpenOptions::new().write(true).read(true).create(true).truncate(true).open(FILENAME);

    match file {
        Ok(mut file) => {
            let mut countries = serde_json::from_str::<Vec<Country>>(&mut buffer).unwrap_or_else(|_| vec![]);
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
    let mut buffer = fs::read_to_string(FILENAME).unwrap_or_else(|_| "[]".to_string());
    let countries = serde_json::from_str::<Vec<Country>>(&mut buffer).unwrap_or_else(|_| vec![]);

    Ok(Output::List(countries))
}
