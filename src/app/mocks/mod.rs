use std::collections::HashMap;

use crate::app::types::{Countries, Country, Name};

use super::types::Maps;

pub struct MockedCountries {
    pub all: Countries,
    pub france: Country,
    pub spain: Country,
    pub greece: Country,
}

#[allow(dead_code)]
pub fn get_mocked_countries() -> MockedCountries {
    let france = Country {
        name: Name {
            common: "France".to_string(),
        },
        flag: "🇫🇷".to_string(),
        alt_spellings: vec!["fr".to_string()],
        area: 500.0,
        tlds: Some(vec![".fr".to_string()]),
        population: 100,
        continents: vec!["Europe".to_string()],
        latlng: Some(vec![10.0, 10.0]),
        maps: Maps {
            google_maps: "https://www.google.com/".to_string(),
        },
        languages: Some(HashMap::new()),
        borders: Some(vec!["SPA".to_string()]),
        cca3: "FRA".to_string(),
    };

    let spain = Country {
        name: Name {
            common: "Spain".to_string(),
        },
        flag: "🇪🇸".to_string(),
        alt_spellings: vec!["sp".to_string()],
        area: 750.0,
        tlds: Some(vec![".es".to_string()]),
        population: 200,
        continents: vec!["Europe".to_string()],
        latlng: Some(vec![10.0, 10.0]),
        maps: Maps {
            google_maps: "https://www.google.com/".to_string(),
        },
        languages: Some(HashMap::new()),
        borders: Some(vec!["FRA".to_string()]),
        cca3: "SPA".to_string(),
    };

    let greece = Country {
        name: Name {
            common: "Greece".to_string(),
        },
        flag: "🇬🇷".to_string(),
        alt_spellings: vec!["gr".to_string()],
        area: 100.0,
        tlds: Some(vec![".gr".to_string()]),
        population: 300,
        continents: vec!["Europe".to_string()],
        latlng: Some(vec![10.0, 10.0]),
        maps: Maps {
            google_maps: "https://www.google.com/".to_string(),
        },
        languages: Some(HashMap::new()),
        borders: Some(vec![]),
        cca3: "GRE".to_string(),
    };

    MockedCountries {
        all: vec![france.clone(), spain.clone(), greece.clone()],
        france,
        spain,
        greece,
    }
}
