const COUNTRY_URL: &str = "https://restcountries.com/v3.1/all";

use crate::app::types::{Countries};

pub async fn get_countries() -> Option<Countries> {
    match reqwest::get(COUNTRY_URL).await.unwrap().json::<Countries>().await {
        Ok(countries) => Some(countries),
        _             => None
    }
}
