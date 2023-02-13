use colored::*;
use figlet_rs::FIGfont;

mod error;
mod info;
mod list;
mod modified;

use crate::app::{
    config::{CMD_ADD, CMD_INFO, CMD_LIST, CMD_REMOVE, JSON_PATH},
    types::Output,
};

pub fn print(output: Output) {
    let font = FIGfont::standard().unwrap();
    let logo = font.convert("BeenThere.").unwrap();
    println!("{logo}");

    match output {
        Output::Add(country) => modified::render_added(&country),
        Output::Remove(country) => modified::render_removed(&country),
        Output::List(model) => list::render(&model),
        Output::Info(model) => info::render(&model),

        Output::Invalid(name) => {
            let name = name.white();
            error::render(&format!("Invalid country: {name}."))
        }
        Output::Unwritable => {
            let filename = JSON_PATH.as_str().white();
            error::render(&format!("Cannot write countries to: {filename}."))
        }
        Output::Unfetchable => error::render("Cannot fetch countries at the moment."),
        Output::Unactionable => {
            let add = CMD_ADD.white();
            let remove = CMD_REMOVE.white();
            let list = CMD_LIST.white();
            let info = CMD_INFO.white();

            error::render(&format!(
                "Invalid command supplied, try: {add}, {remove}, {list} or {info}."
            ))
        }
    }

    println!("\n");
}
