use crate::app::types::{Country};

pub fn add(country: Country) -> Result<(), String> {
    println!("Adding {:?}", country);
    Ok(())
}
