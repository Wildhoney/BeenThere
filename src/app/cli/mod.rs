use clap::{arg, Command};

use crate::app::fs::{add, info, list, remove};
use crate::app::types::Output;

pub const FILENAME: &str = "been-there.json";

pub const CMD_ADD: &str = "add";

pub const CMD_REMOVE: &str = "remove";

pub const CMD_LIST: &str = "list";

pub const CMD_INFO: &str = "info";

pub async fn run() -> Output {
    match crate::app::utils::get_countries_from_remote().await {
        Some(countries) => match get_args().get_matches().subcommand() {
            Some((action @ CMD_ADD, arg))
            | Some((action @ CMD_REMOVE, arg))
            | Some((action @ CMD_INFO, arg)) => {
                let name = arg.get_one::<String>("NAME").unwrap();
                let country = crate::app::utils::get_country_by_name(name, &countries);

                match country {
                    Some(country) => match action {
                        CMD_ADD => add(FILENAME, &country, &countries),
                        CMD_REMOVE => remove(FILENAME, &country, &countries),
                        CMD_INFO => info(FILENAME, &country, &countries),
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

pub fn get_args() -> Command {
    Command::new("been-there")
        .about("Terminal application for listing the countries you've visited with other interesting statistics thrown in.")
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
                .alias("rm")
                .about("Remove a country from your list")
                .arg(arg!(<NAME> "Name of the country"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new(CMD_LIST)
                .alias("ls")
                .about("List out all of the countries you have visited")
        )
        .subcommand(
            Command::new(CMD_INFO)
                .alias("i")
                .about("Display information about a particular country")
                .arg(arg!(<NAME> "Name of the country"))
                .arg_required_else_help(true),
        )
}
