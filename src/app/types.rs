use serde::{Deserialize, Serialize};

pub type Countries = Vec<Country>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Country {
    pub name: Name,
    pub flag: String,
    pub population: usize,
    pub area: f64,
    pub continents: Vec<String>,
    pub tld: Option<Vec<String>>,
    pub latlng: Option<Vec<f64>>,
    pub maps: Maps,

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

#[derive(Debug, PartialEq)]
pub enum Output {
    Add(Country),
    Remove(Country),
    List(Countries),
    Info(Country),
    Invalid(String),
    Unwritable,
    Unfetchable,
    Unactionable,
}
