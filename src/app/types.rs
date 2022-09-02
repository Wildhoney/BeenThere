use serde::{Serialize, Deserialize};

pub type Countries = Vec<Country>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Country {
    pub name: Name,
    pub flag: String,
    pub cioc: Option<String>
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
    Noop
}