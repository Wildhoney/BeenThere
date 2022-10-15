use clap::{arg, Command};

use crate::app::manager::{add, list, remove};
use crate::app::types::Output;

pub const FILENAME: &str = "been-there.json";

pub const CMD_ADD: &str = "add";

pub const CMD_REMOVE: &str = "remove";

pub const CMD_LIST: &str = "list";

pub async fn run() -> Output {
    match crate::app::utils::get_countries_from_remote().await {
        Some(countries) => match get_args().get_matches().subcommand() {
            Some((action @ CMD_ADD, arg)) | Some((action @ CMD_REMOVE, arg)) => {
                let name = arg.get_one::<String>("NAME").unwrap();
                let country = crate::app::utils::get_country_by_name(name, &countries);

                match country {
                    Some(country) => match action {
                        CMD_ADD => add(FILENAME, &country, &countries),
                        CMD_REMOVE => remove(FILENAME, &country, &countries),
                        _ => Output::Unactionable,
                    },
                    None => Output::Invalid(name.to_string()),
                }
            }
            Some((CMD_LIST, _)) => list(FILENAME, &countries),
            _ => Output::Unactionable,
        },
        None => Output::Unfetchable,
    }
}

pub fn get_args() -> Command<'static> {
    Command::new("been-there")
        .about("Terminal application for listing the countries you've visited with other interesting statistics thrown in")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new(CMD_ADD)
                .about("Add another country to your list")
                .arg(arg!(<NAME> "Name of the country"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new(CMD_REMOVE)
                .about("Remove a country from your list")
                .arg(arg!(<NAME> "Name of the country"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new(CMD_LIST)
                .about("List out all of the countries you have visited")
        )
}
