use crate::app::utils::{get_countries_by_land, get_visited_continents};
use crate::app::{types::Countries, utils::get_countries_by_people};
use colored::*;
use num_format::Locale;
use num_format::ToFormattedString;
use term_table::row::Row;
use term_table::table_cell::TableCell;
use term_table::{Table, TableStyle};

pub fn render(countries: &Countries) {
    if countries.len() == 0 {
        let any = "any".bold();
        let home_country = "Not even your home country...".white();
        println!("You haven't yet been to {any} countries! {home_country}");

        return;
    }

    let count = countries.len().to_string().bold().cyan();
    println!("You have visited {count} countries!\n",);

    render_people(&countries);
    println!("\n");

    render_land(&countries);
    println!("\n");

    render_continents(&countries);
    println!("\n");

    render_countries(&countries);
}

fn render_people(countries: &Countries) {
    let (fewest_people, most_people) = get_countries_by_people(countries);
    let data = [("Most", most_people), ("Fewest", fewest_people)];
    let dimmed = "┃".dimmed();

    for (prefix, country) in data {
        let name = country.name.common.white();
        let flag = &country.flag;
        let population = country.population.to_formatted_string(&Locale::en).white();
        let prefix = prefix.bold();
        let suffix = "ppl".dimmed();

        println!("{dimmed} {prefix} people: {name} {flag}  ({population} {suffix})");
    }
}

fn render_land(countries: &Countries) {
    let (least_land, most_land) = get_countries_by_land(countries);
    let data = [("Most", most_land), ("Least", least_land)];
    let dimmed = "┃".dimmed();

    for (prefix, country) in data {
        let name = country.name.common.white();
        let flag = &country.flag;
        let population = country.population.to_formatted_string(&Locale::en).white();
        let prefix = prefix.bold();
        let suffix = "km2".dimmed();

        println!("{dimmed} {prefix} land: {name} {flag}  ({population} {suffix})");
    }
}

fn render_continents(countries: &Countries) {
    let continents = get_visited_continents(&countries);
    let title = "Continents".bold();
    let dimmed = "┃".dimmed();

    print!("{dimmed} {title}: ");

    continents.into_iter().for_each(|(continent, count)| {
        let bullet = "◦".dimmed();
        let count = count.to_string().white();
        let open_parenthesis = "(".dimmed();
        let closed_parenthesis = ")".dimmed();

        print!("{bullet} {continent} {open_parenthesis}{count}{closed_parenthesis} ");
    });
}

fn render_countries(countries: &Countries) {
    println!("\n{}:", "Countries".white());

    let mut countries = countries.clone();
    countries.sort_by(|a, b| {
        a.name
            .common
            .to_lowercase()
            .cmp(&b.name.common.to_lowercase())
    });

    let mut table = Table::new();
    table.separate_rows = false;
    table.style = TableStyle::blank();

    for countries in countries.chunks(4) {
        let countries = countries
            .iter()
            .map(|country| {
                let bullet = "◦".dimmed();
                let flag = &country.flag;
                let name = &country.name.common;

                TableCell::new(format!("{bullet} {flag}  {name}"))
            })
            .collect::<Vec<_>>();

        table.add_row(Row::new(countries));
    }

    let output = table.render();
    println!("{output}");
}
