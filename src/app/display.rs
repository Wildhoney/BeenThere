use figlet_rs::FIGfont;
use colored::*;

use crate::app::types::Output;

pub fn render(output: Result<Output, String>) -> () {
    let standard_font = FIGfont::standand().unwrap();
    println!("{}", standard_font.convert("BeenThere.").unwrap());

    match output {
        Ok(output) => match output {
            Output::Add(country)    => println!("Added {} {}", country.name.common, country.flag),
            Output::Remove(country) => println!("Removed {} {}", country.name.common, country.flag),
            Output::List(countries) => {
                println!("You have been to {} countries:\n", countries.len().to_string().bold());
                
                let mut countries = countries.clone();
                countries.sort_by(|a, b| a.name.common.to_lowercase().cmp(&b.name.common.to_lowercase()));

                countries.into_iter().for_each(|country| {
                    println!("{} {}  {}", "•".white(), country.flag, country.name.common);
                });
            },
            Output::Noop            => println!("Ok")
        },
        Err(message) => {
            println!("{}", "Oh no! An error has occurred...".bright_red().bold());
            println!("{}", message.white());
        }
    }
}