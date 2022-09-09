use serde::{Serialize, Deserialize};

pub type Countries = Vec<Country>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Country {
    pub name: Name,
    pub flag: String,
    pub population: usize,
    pub area: f64,
    pub continents: Vec<String>,

    #[serde(rename(deserialize = "altSpellings"))]
    pub alt_spellings: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Name {
    pub common: String
}

pub type Continents = Vec<(String, usize)>;

#[derive(Debug, PartialEq)]
pub enum Output {
    Add(Country),
    Remove(Country),
    List(Countries),
    Invalid(String),
    Unwritable,
    Unfetchable,
    Unactionable
}