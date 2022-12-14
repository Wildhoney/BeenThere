use crate::app::types::List;
use crate::app::utils::{get_countries_by_land, get_visited_continents, pluralise};
use crate::app::{types::Countries, utils::get_countries_by_people};
use colored::*;
use num_format::Locale;
use num_format::ToFormattedString;
use term_table::row::Row;
use term_table::table_cell::TableCell;
use term_table::{Table, TableStyle};

pub fn render(model: &List) {
    match model.visited_countries.len() {
        0 => render_visited_nowhere(),
        _ => render_visited_somewhere(&model.visited_countries, model.total_countries),
    }
}

fn render_visited_nowhere() {
    let any = "any".bold();
    let home_country = "Not even your home country...".white();
    println!("You haven't yet been to {any} countries! {home_country}");
}

fn render_visited_somewhere(countries: &Countries, total_countries: usize) {
    let count = countries.len().to_string().bold().cyan();
    let suffix = pluralise(countries.len(), "country", "countries");
    let value = ((countries.len() as f64) / (total_countries as f64)) * 100.0;
    println!("You have visited {count} {suffix}!");

    let value = format!("🌍 That's {:.1}% of {total_countries} countries.\n", value).dimmed();
    println!("{value}\n");

    render_people(&countries);
    println!("\n");

    render_land(&countries);
    println!("\n");

    render_continents(&countries);
    println!("\n\n");

    render_countries(&countries);
}

fn render_people(countries: &Countries) {
    let (fewest_people, most_people) = get_countries_by_people(countries);
    let line = "┃".dimmed();

    for (prefix, country) in [("Most", most_people), ("Fewest", fewest_people)] {
        let name = country.name.common.white();
        let flag = &country.flag;
        let population = country.population.to_formatted_string(&Locale::en).white();
        let prefix = prefix.bold();
        let suffix = "ppl".dimmed();

        println!("{line} {prefix} people: {name} {flag}  ({population} {suffix})");
    }
}

fn render_land(countries: &Countries) {
    let (least_land, most_land) = get_countries_by_land(countries);
    let line = "┃".dimmed();

    for (prefix, country) in [("Most", most_land), ("Least", least_land)] {
        let name = country.name.common.white();
        let flag = &country.flag;
        let population = country.population.to_formatted_string(&Locale::en).white();
        let prefix = prefix.bold();
        let suffix = "km2".dimmed();

        println!("{line} {prefix} land: {name} {flag}  ({population} {suffix})");
    }
}

fn render_continents(countries: &Countries) {
    let continents = get_visited_continents(&countries);
    let title = pluralise(countries.len(), "Continent", "Continents").bold();
    let line = "┃".dimmed();
    print!("{line} {title}: ");

    continents.into_iter().for_each(|(continent, count)| {
        let bullet = "◦".dimmed();
        let count = count.to_string().white();
        let open_parenthesis = "(".dimmed();
        let closed_parenthesis = ")".dimmed();

        print!("{bullet} {continent} {open_parenthesis}{count}{closed_parenthesis} ");
    });
}

fn render_countries(countries: &Countries) {
    let title = "Countries".white();
    println!("{title}:");

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
