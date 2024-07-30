use std::collections::HashMap;

use crate::{country_data::CountryData, medals::Medals};

#[derive(serde::Serialize)]
pub struct MedalsRecord {
    country_code: String,
    country_name: String,
    gdp: u64,
    population: u64,
    gdp_per_head: u64,
    gold: u32,
    silver: u32,
    bronze: u32,
    total: u32,
    // gold_by_gdp: u32,
    // silver_by_gdp: u32,
    // bronze_by_gdp: u32,
    // total_by_gdp: u32,
}

pub fn assemble_table(
    country_data: HashMap<String, CountryData>,
    medals: Vec<Medals>,
) -> Vec<MedalsRecord> {
    let mut medals_table = Vec::new();

    for country in medals {
        println!(
            "Country: {} :: {}",
            &country.country_code, country.country_name
        );
        let country_code = match country_data.get(&country.country_code) {
            Some(_) => &country.country_code,
            None => &fix_country_code(&country.country_code),
        };

        let gdp = country_data.get(country_code).unwrap().gdp;
        let population = country_data.get(country_code).unwrap().population;
        let medals_record = MedalsRecord {
            country_code: country.country_code.clone(),
            country_name: country_data.get(country_code).unwrap().country_name.clone(),
            gdp,
            population,
            gdp_per_head: gdp / population,
            gold: country.gold,
            silver: country.silver,
            bronze: country.bronze,
            total: country.total,
        };
        medals_table.push(medals_record);
    }

    medals_table
}

fn fix_country_code(code: &str) -> String {
    match code {
        "GER" => "DEU".to_string(),
        "RSA" => "ZAF".to_string(),
        "ROM" => "ROU".to_string(),
        "TKE" => "TUR".to_string(),
        "FIJ" => "FJI".to_string(),
        "KOS" => "XKX".to_string(),
        "MGL" => "MNG".to_string(),
        "CRO" => "HRV".to_string(),
        "EIR" => "IRL".to_string(),
        "SUI" => "CHE".to_string(),
        _ => "XXX".to_string(),
    }
}
