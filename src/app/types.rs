use serde::Deserialize;

pub type Countries = Vec<Country>;

#[derive(Deserialize, Debug)]
pub struct Country {
    pub name: Name
}

#[derive(Deserialize, Debug)]
pub struct Name {
    pub common: String
}