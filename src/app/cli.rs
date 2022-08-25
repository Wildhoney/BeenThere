use clap::{arg, Command};

pub async fn run() -> Result<(), String> {
    let matches = get_args().get_matches();
    
    match matches.subcommand() {
        Some((action @ "add", arg)) | Some((action @ "rm", arg)) => {
            let countries = crate::app::resources::get_countries().await.unwrap();
            let name      = arg.get_one::<String>("NAME").unwrap();
            let country   = crate::app::utils::get_country(name, countries)?;

            match action {
                "add" => crate::app::manager::add(country),
                "rm"  => crate::app::manager::add(country),
                _     => Err(format!("Invalid action: {}", action))
            }
        },
        Some(("stats", _)) => {
            println!("Lots of lovely stats!");
            Ok(())
        },
        _ => {
            println!("I dunno");
            Ok(())
        }
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
            Command::new("stats")
                .about("List out all the statistics related to your travels")
        )
}
