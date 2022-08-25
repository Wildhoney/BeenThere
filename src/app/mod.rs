use clap::{arg, Command};

const PARAM: &str = "NAME";

pub fn run() -> () {
    let matches = get_args().get_matches();

    match matches.subcommand() {
        Some(("add", arg)) => {
            let name = arg.get_one::<String>(PARAM).unwrap();
            println!("Adding {}", name);
        },
        Some(("rm", arg)) => {
            let name = arg.get_one::<String>(PARAM).unwrap();
            println!("Removing {}", name);
        },
        Some(("stats", _)) => {
            println!("Lots of lovely stats!");
        },
        _ => println!("I dunno")
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
