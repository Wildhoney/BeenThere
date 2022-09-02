use figlet_rs::FIGfont;

use crate::app::types::Output;

pub fn render(output: Result<Output, String>) -> () {
    let standard_font = FIGfont::standand().unwrap();
    println!("{}", standard_font.convert("BeenThere.").unwrap());

    match output {
        Ok(output) => match output {
            Output::Add(country)    => println!("Added {}", country.name.common),
            Output::List(countries) => {
                countries.into_iter().for_each(|country| {
                    println!("• {}", country.name.common);
                });
            },
            Output::Noop            => println!("Ok")
        },
        Err(message) => println!("{:?}", message)
    }
}