use figlet_rs::FIGfont;
use colored::*;
use num_format::{Locale, ToFormattedString};

use crate::app::types::{Country, Output};

pub fn render(output: Output) -> () {
    let standard_font = FIGfont::standand().unwrap();
    println!("{}", standard_font.convert("BeenThere.").unwrap());

    match output {
        Output::Add(country)    => print_added_or_removed("Added".to_string(), country),
        Output::Remove(country) => print_added_or_removed("Removed".to_string(), country),
        Output::List(countries) => {

            match countries.len() {
                0 => println!("You haven't yet been to {} countries! {}", "any".bold(), "Not even your home country...".white()),
                _ => {
                    println!("You have been to {} countries:\n", countries.len().to_string().bold());
            
                    let mut countries = countries.clone();
                    countries.sort_by(|a, b| a.name.common.to_lowercase().cmp(&b.name.common.to_lowercase()));
        
                    countries.into_iter().for_each(|country| {
                        println!("{} {}  {}", "•".white(), country.flag, country.name.common);
                    });
                }
            }
        },
        Output::Invalid(name) => print_error(format!("Invalid country: {}", name.white())),
        Output::Noop          => print_error("Invalid command".to_string())
    }

    println!("\n");
}

fn print_added_or_removed(prefix: String, country: Country) -> () {
    let area = match country.area {
        Some(area) => format!("within {} km2.", (area as usize).to_formatted_string(&Locale::en).to_string().white()),
        None       => format!(".")
    };

    println!(
        "{} {} {}  which has a population of {} people {}",
        prefix, country.name.common.white(), country.flag,
        country.population.to_formatted_string(&Locale::en).to_string().white(), area
    );
}

fn print_error(message: String) -> () {
    println!("{}", "Oh no! An error has occurred...".bright_red().bold());
    println!("{}", message);
}
