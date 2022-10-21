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

    pub latlng: Option<Vec<f64>>,
    pub maps: Maps,
    pub languages: Option<HashMap<String, String>>,
    pub borders: Option<Vec<String>>,

    #[serde(rename(deserialize = "tld"))]
    pub tlds: Option<Vec<String>>,

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
pub struct Info {
    pub country: Country,
    pub visited_countries: Countries,
    pub has_visited: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub visited_countries: Countries,
    pub total_countries: usize,
}

#[derive(Debug, PartialEq)]
pub enum Output {
    Add(Country),
    Remove(Country),
    List(List),
    Info(Info),
    Invalid(String),
    Unwritable,
    Unfetchable,
    Unactionable,
}
