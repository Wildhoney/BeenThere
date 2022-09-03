use serde::{Serialize, Deserialize};

pub type Countries = Vec<Country>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Country {
    pub name: Name,
    pub flag: String,
    pub population: usize,
    pub area: f64,

    #[serde(rename(deserialize = "altSpellings"))]
    pub alt_spellings: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Name {
    pub common: String
}

#[derive(Debug)]
pub enum Output {
    Add(Country),
    Remove(Country),
    List(Countries),
    Invalid(String),
    Unwritable,
    Noop
}