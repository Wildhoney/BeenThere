use crate::app::types::{Countries, Country, Name};

#[allow(dead_code)]
pub fn get_mock_countries() -> (Countries, Country, Country, Country) {
    let france = Country {
        name: Name { common: "France".to_string() },
        flag: "🇫🇷".to_string(),
        alt_spellings: vec!["fr".to_string()],
        area: 500.0,
        population: 100
    };
    
    let spain  = Country {
        name: Name { common: "Spain".to_string() },
        flag: "🇪🇸".to_string(),
        alt_spellings: vec!["sp".to_string()],
        area: 750.0,
        population: 200
    };
    
    let greece = Country {
        name: Name { common: "Greece".to_string() },
        flag: "🇬🇷".to_string(),
        alt_spellings: vec!["gr".to_string()],
        area: 100.0,
        population: 300
    };
    
    (vec![france.clone(), spain.clone(), greece.clone()], france, spain, greece)
}
