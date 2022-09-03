use clap::{arg, Command};

use crate::app::types::Output;
use crate::app::manager::{add, remove, list};

pub async fn run() -> Output {
    let matches   = get_args().get_matches();
    let countries = crate::app::utils::get_countries_from_remote().await.unwrap();
    
    match matches.subcommand() {
        Some((action @ "add", arg)) | Some((action @ "rm", arg)) => {
            let name    = arg.get_one::<String>("NAME").unwrap();
            let country = crate::app::utils::get_country_by_name(name, countries.clone());

            match country {
                Some(country) => {
                    match action {
                        "add" => add(country, countries.clone()),
                        "rm"  => remove(country, countries.clone()),
                        _     => Output::Noop
                    }
                },
                None => Output::Invalid(name.to_string())
            }
        },
        Some(("ls", _)) => list(countries.clone()),
        _               => Output::Noop
    }
}

pub fn get_args() -> Command<'static> {
    Command::new("been-there")
        .about("Terminal application for listing the countries you've visited with other interesting statistics thrown in")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("add")
                .about("Add another country to your list")
                .arg(arg!(<NAME> "Name of the country"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a country from your list")
                .arg(arg!(<NAME> "Name of the country"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("ls")
                .about("List out all of the countries you have visited")
        )
}
