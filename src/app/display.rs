use figlet_rs::FIGfont;
use colored::*;

use crate::app::{types::{Output}, cli::FILENAME};

pub fn render(output: Output) {
    let standard_font = FIGfont::standard().unwrap();
    println!("{}", standard_font.convert("BeenThere.").unwrap());

    match output {
        Output::Add(country)    => print::added_or_removed("Added", &country),
        Output::Remove(country) => print::added_or_removed("Removed", &country),
        Output::List(countries) => print::list(&countries),
        Output::Invalid(name)   => print::error(&format!("Invalid country: {}.", name.white())),
        Output::Unwritable      => print::error(&format!("Cannot write countries to: {}.", FILENAME.white())),
        Output::Unfetchable     => print::error("Cannot fetch countries at the moment."),
        Output::Unactionable    => print::error(&format!("Invalid command supplied, try: {}, {} or {}.", "add".white(), "rm".white(), "ls".white()))
    }

    println!("\n");
}

mod print {
    use term_table::{Table, TableStyle, row::Row, table_cell::TableCell};
    use num_format::{Locale, ToFormattedString};
    use colored::*;

    use crate::app::{types::{Countries, Country, Continents}, utils::{get_countries_by_people, get_countries_by_land, get_visited_continents}};

    pub fn list(countries: &Countries) {
        match countries.len() {
            0 => println!("You haven't yet been to {} countries! {}", "any".bold(), "Not even your home country...".white()),
            _ => {
                println!("You have visited {} countries!\n", countries.len().to_string().bold().cyan());
    
                let (fewest_people, most_people) = get_countries_by_people(countries);
                people("Most", &most_people);
                people("Fewest", &fewest_people);
                println!("\n");
    
                let (least_land, most_land) = get_countries_by_land(countries);
                land("Most", &most_land);
                land("Least", &least_land);

                println!("\n");
                continents(get_visited_continents(countries));
                println!("\n");

                println!("\n{}:", "Countries".white());

                let mut countries = countries.clone();
                countries.sort_by(|a, b| a.name.common.to_lowercase().cmp(&b.name.common.to_lowercase()));
    
                let mut table       = Table::new();
                table.separate_rows = false;
                table.style         = TableStyle::blank(); 

                for countries in countries.chunks(4) {
                    let countries = countries.iter().map(|country| {
                        TableCell::new(format!("{} {}  {}", "◦".dimmed(), country.flag, country.name.common))
                    }).collect::<Vec<_>>();
    
                    table.add_row(Row::new(countries));
                }
    
                println!("{}", table.render());
            }
        }
    }

    pub fn people(quantifier: &str, country: &Country) {
        println!(
            "{} {} people: {} {}  ({} {})", "┃".dimmed(), quantifier.bold(),
            country.name.common.white(), country.flag,
            country.population.to_formatted_string(&Locale::en).white(),
            "ppl".dimmed()
        );
    }
    
    pub fn land(quantifier: &str, country: &Country) {
        println!(
            "{} {} land: {} {}  ({} {})", "┃".dimmed(), quantifier.bold(),
            country.name.common.white(), country.flag,
            (country.area as usize).to_formatted_string(&Locale::en).white(),
            "km2".dimmed()
        );
    }

    pub fn continents(continents: Continents) {
        print!("{} {}: ", "┃".dimmed(), "Continents".bold());

        continents.into_iter().for_each(|(continent, count)| {
            print!("{} {} {}{}{} ", "◦".dimmed(), continent, "(".dimmed(), count.to_string().white(), ")".dimmed());
        });
    }
    
    pub fn added_or_removed(prefix: &str, country: &Country) {
        println!(
            "{} {} {}  which has a population of {} people within {} km2.",
            prefix, country.name.common.white(), country.flag,
            country.population.to_formatted_string(&Locale::en).white(),
            (country.area as usize).to_formatted_string(&Locale::en).white()
        );
    }
    
    pub fn error(message: &str) {
        println!("{}", "Oh no! An error has occurred...".bright_red().bold());
        println!("{}", message);
    }
}
