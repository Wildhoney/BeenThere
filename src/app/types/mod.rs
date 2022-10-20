use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type Countries = Vec<Country>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Country {
    pub name: Name,
    pub cca3: String,
    pub flag: String,
    pub population: usize,
    pub area: f64,
    pub continents: Vec<String>,
    pub tld: Option<Vec<String>>,
    pub latlng: Option<Vec<f64>>,
    pub maps: Maps,
    pub languages: Option<HashMap<String, String>>,
    pub borders: Option<Vec<String>>,

    #[serde(rename(deserialize = "altSpellings"))]
    pub alt_spellings: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Name {
    pub common: String,
}

pub type Continents = Vec<(String, usize)>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Maps {
    #[serde(rename(deserialize = "googleMaps"))]
    pub google_maps: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    pub country: Country,
    pub countries: Countries,
    pub has_visited: bool,
}

#[derive(Debug, PartialEq)]
pub enum Output {
    Add(Country),
    Remove(Country),
    List(Countries),
    Info(Stats),
    Invalid(String),
    Unwritable,
    Unfetchable,
    Unactionable,
}
