use figlet_rs::FIGfont;
use colored::*;

use crate::app::types::Output;

pub fn render(output: Output) -> () {
    let standard_font = FIGfont::standand().unwrap();
    println!("{}", standard_font.convert("BeenThere.").unwrap());

    match output {
        Output::Add(country)    => println!("Added {} {}", country.name.common, country.flag),
        Output::Remove(country) => println!("Removed {} {}", country.name.common, country.flag),
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
        Output::Invalid(name) => error(format!("Invalid country: {}", name.white())),
        Output::Noop          => error("Invalid command".to_string())
    }

    println!("\n");
}

fn error(message: String) -> () {
    println!("{}", "Oh no! An error has occurred...".bright_red().bold());
    println!("{}", message);
}
