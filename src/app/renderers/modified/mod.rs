use crate::app::types::Country;
use colored::*;
use num_format::{Locale, ToFormattedString};

pub fn render_added(country: &Country) {
    render("Added", &country);
}

pub fn render_removed(country: &Country) {
    render("Removed", &country);
}

fn render(prefix: &str, country: &Country) {
    let name = country.name.common.white();
    let flag = &country.flag;
    let population = country.population.to_formatted_string(&Locale::en).white();
    let area = (country.area as usize)
        .to_formatted_string(&Locale::en)
        .white();

    println!(
        "{prefix} {name} {flag}  which has a population of {population} people within {area} km2."
    );
}
