use figlet_rs::FIGfont;
use colored::*;

use crate::app::{types::{Output}, cli::FILENAME};

pub fn render(output: Output) -> () {
    let standard_font = FIGfont::standand().unwrap();
    println!("{}", standard_font.convert("BeenThere.").unwrap());

    match output {
        Output::Add(country)    => print::added_or_removed("Added".to_string(), country),
        Output::Remove(country) => print::added_or_removed("Removed".to_string(), country),
        Output::List(countries) => print::list(countries),
        Output::Invalid(name)   => print::error(format!("Invalid country: {}.", name.white())),
        Output::Unwritable      => print::error(format!("Cannot write countries to: {}.", FILENAME.white())),
        Output::Unfetchable     => print::error("Cannot fetch countries at the moment.".to_string()),
        Output::Unactionable    => print::error(format!("Invalid command supplied, try: {}, {} or {}.", "add".white(), "rm".white(), "ls".white()))
    }

    println!("\n");
}

mod print {
    use term_table::{Table, TableStyle, row::Row, table_cell::{TableCell, Alignment}};
    use num_format::{Locale, ToFormattedString};
    use colored::*;

    use crate::app::{types::{Countries, Country}, utils::{get_countries_by_people, get_countries_by_land}};

    pub fn list(countries: Countries) -> () {
        match countries.len() {
            0 => println!("You haven't yet been to {} countries! {}", "any".bold(), "Not even your home country...".white()),
            _ => {
                println!("You have visited {} countries!\n", countries.len().to_string().bold().cyan());
    
                let (most_people, fewest_people) = get_countries_by_people(&countries);
                people("Most".to_string(), most_people);
                people("Fewest".to_string(), fewest_people);
                println!("\n");
    
                let (most_land, least_land) = get_countries_by_land(&countries);
                land("Most".to_string(), most_land);
                land("Least".to_string(), least_land);
                println!("\n{}:", "Countries".white());
        
                let mut countries = countries.clone();
                countries.sort_by(|a, b| a.name.common.to_lowercase().cmp(&b.name.common.to_lowercase()));
    
                let mut table       = Table::new();
                table.separate_rows = false;
                table.style         = TableStyle::blank(); 

                for countries in countries.chunks(4) {
                    let countries = countries.into_iter().map(|country| {
                        TableCell::new_with_alignment(format!("{} {}  {}", "◦".dimmed(), country.flag, country.name.common), 2, Alignment::Left)
                    }).collect::<Vec<_>>();
    
                    table.add_row(Row::new(countries));
                }
    
                println!("{}", table.render());
            }
        }
    }

    pub fn people(quantifier: String, country: Country) -> () {
        println!(
            "{} {} people: {} {}  ({} {})", "┃".dimmed(), quantifier.bold(),
            country.name.common.white(), country.flag,
            country.population.to_formatted_string(&Locale::en).to_string().white(),
            "ppl".dimmed()
        );
    }
    
    pub fn land(quantifier: String, country: Country) -> () {
        println!(
            "{} {} land: {} {}  ({} {})", "┃".dimmed(), quantifier.bold(),
            country.name.common.white(), country.flag,
            (country.area as usize).to_formatted_string(&Locale::en).to_string().white(),
            "km2".dimmed()
        );
    }
    
    pub fn added_or_removed(prefix: String, country: Country) -> () {
        println!(
            "{} {} {}  which has a population of {} people {}",
            prefix, country.name.common.white(), country.flag,
            country.population.to_formatted_string(&Locale::en).to_string().white(),
            format!("within {} km2.", (country.area as usize).to_formatted_string(&Locale::en).to_string().white())
        );
    }
    
    pub fn error(message: String) -> () {
        println!("{}", "Oh no! An error has occurred...".bright_red().bold());
        println!("{}", message);
    }
}
