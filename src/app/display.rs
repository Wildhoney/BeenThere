use figlet_rs::FIGfont;
use colored::*;
use num_format::{Locale, ToFormattedString};

use crate::app::{types::{Country, Output}, utils::{get_countries_by_people, get_countries_by_land}};

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
                    println!("You have visited {} countries!\n", countries.len().to_string().bold().cyan());

                    let (most_people, fewest_people) = get_countries_by_people(&countries);
                    print_statistic_country_people("Most".to_string(), most_people);
                    print_statistic_country_people("Fewest".to_string(), fewest_people);
                    println!("\n");

                    let (most_land, least_land) = get_countries_by_land(&countries);
                    print_statistic_country_land("Most".to_string(), most_land);
                    print_statistic_country_land("Least".to_string(), least_land);
                    println!("\n");
            
                    let mut countries = countries.clone();
                    countries.sort_by(|a, b| a.name.common.to_lowercase().cmp(&b.name.common.to_lowercase()));
                    countries.into_iter().for_each(|country| {
                        println!("{} {}  {}", "◦".dimmed(), country.flag, country.name.common);
                    });
                }
            }
        },
        Output::Invalid(name) => print_error(format!("Invalid country: {}", name.white())),
        Output::Noop          => print_error("Invalid command".to_string())
    }

    println!("\n");
}

fn print_statistic_country_people(quantifier: String, country: Country) -> () {
    println!(
        "{} {} people: {} {}  ({} {})", "┃".dimmed(), quantifier.bold(),
        country.name.common.white(), country.flag,
        country.population.to_formatted_string(&Locale::en).to_string().white(),
        "ppl".dimmed()
    );
}

fn print_statistic_country_land(quantifier: String, country: Country) -> () {
    println!(
        "{} {} land: {} {}  ({} {})", "┃".dimmed(), quantifier.bold(),
        country.name.common.white(), country.flag,
        (country.area as usize).to_formatted_string(&Locale::en).to_string().white(),
        "km2".dimmed()
    );
}

fn print_added_or_removed(prefix: String, country: Country) -> () {
    println!(
        "{} {} {}  which has a population of {} people {}",
        prefix, country.name.common.white(), country.flag,
        country.population.to_formatted_string(&Locale::en).to_string().white(),
        format!("within {} km2.", (country.area as usize).to_formatted_string(&Locale::en).to_string().white())
    );
}

fn print_error(message: String) -> () {
    println!("{}", "Oh no! An error has occurred...".bright_red().bold());
    println!("{}", message);
}
