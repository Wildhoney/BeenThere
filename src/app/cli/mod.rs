use clap::{arg, Command};

use crate::app::fs::{add, info, list, remove};
use crate::app::types::Output;

use super::config::{CMD_ADD, CMD_INFO, CMD_LIST, CMD_REMOVE, JSON_PATH, PKG_VERSION};

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
                        CMD_ADD => add(JSON_PATH.as_str(), &country, &countries),
                        CMD_REMOVE => remove(JSON_PATH.as_str(), &country, &countries),
                        CMD_INFO => info(JSON_PATH.as_str(), &country, &countries),
                        _ => Output::Unactionable,
                    },
                    None => Output::Invalid(name.to_string()),
                }
            }
            Some((CMD_LIST, _)) => list(JSON_PATH.as_str(), &countries),
            _ => Output::Unactionable,
        },
        None => Output::Unfetchable,
    }
}

pub fn get_args() -> Command {
    Command::new("been-there")
    .version(PKG_VERSION)
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
