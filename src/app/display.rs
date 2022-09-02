use crate::app::types::Output;

pub fn render(output: Result<Output, String>) -> () {
    match output {
        Ok(output) => match output {
            Output::Add(country)    => println!("Added {}", country.name.common),
            Output::List(countries) => println!("Countries you've visited: {:?}", countries),
            Output::Noop            => println!("Ok")
        },
        Err(message) => println!("{:?}", message)
    }
}