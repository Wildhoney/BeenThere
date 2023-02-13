use crate::app::types::Info;
use crate::app::utils::{find_neighbouring_countries_by_cca3, pluralise};
use colored::*;
use itertools::Itertools;
use num_format::Locale;
use num_format::ToFormattedString;
use term_table::row::Row;
use term_table::table_cell::TableCell;
use term_table::{Table, TableStyle};

pub fn render(model: &Info) {
    let name = &model.country.name.common.bold();
    let flag = &model.country.flag;
    let line = "┃".dimmed();
    println!("{flag}  {name}\n");

    let visited = if model.has_visited {
        format!(
            "You're lucky enough to have visited {} already!",
            &model.country.name.common
        )
    } else {
        format!(
            "Unfortunately you have not yet never visited {}.",
            &model.country.name.common
        )
    };

    println!("{}\n", visited.cyan());

    let label = "Population:".white();
    let value = model.country.population.to_formatted_string(&Locale::en);
    let suffix = "ppl".dimmed();
    println!("{line} {label} {value} {suffix}");

    let label = "Area:".white();
    let value = (model.country.area as usize).to_formatted_string(&Locale::en);
    let suffix = "km2".dimmed();
    println!("{line} {label} {value} {suffix}");

    let label = pluralise(model.country.continents.len(), "Continent:", "Continents:").white();
    let value = model.country.continents.iter().join(",");
    println!("{line} {label} {value}");

    if let Some(latlng) = &model.country.latlng {
        let label = "Lat/Lng:".white();
        let values = latlng
            .iter()
            .map(|latlng| latlng.to_string())
            .collect::<Vec<_>>();
        let value = format!(
            "{}{}, {}{}",
            values[0],
            "°N".dimmed(),
            values[1],
            "°S".dimmed()
        );
        let url = format!("({})", &model.country.maps.google_maps.dimmed().underline()).dimmed();
        println!("{line} {label} {value} {url}");
    }

    if let Some(languages) = &model.country.languages {
        let label = pluralise(languages.len(), "Language:", "Languages:").white();
        let value = languages.values().join(", ");
        println!("{line} {label} {value}");
    }

    if let Some(tlds) = &model.country.tlds {
        let label = pluralise(tlds.len(), "TLD:", "TLDs:").white();
        let value = tlds.iter().join(", ");
        println!("{line} {label} {value}");
    }

    if let Some(borders) = &model.country.borders {
        if borders.len() > 0 {
            println!("\nNeighbours:");

            let mut table = Table::new();
            table.separate_rows = false;
            table.style = TableStyle::blank();

            for countries in
                find_neighbouring_countries_by_cca3(borders, &model.visited_countries).chunks(4)
            {
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
    }
}
