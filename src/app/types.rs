use serde::{Serialize, Deserialize};

pub type Countries = Vec<Country>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Country {
    pub name: Name
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Name {
    pub common: String
}

#[derive(Debug)]
pub enum Output {
    Add(Country),
    List(Countries),
    Noop
}