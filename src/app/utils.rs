use crate::app::types::{Countries, Country};

pub fn get_country(country: &String, countries: Countries) -> Result<Country, String> {
    match countries.into_iter().find(|x| &x.name.common == country) {
        Some(country) => Ok(country),
        _ => Err(format!("We're unable to find the country '{}'", country))
    }

}