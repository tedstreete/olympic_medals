use std::{cmp::Ordering, collections::HashMap};

use crate::{country_data::CountryData, medals::Medals};

#[derive(serde::Serialize)]
pub struct MedalsRecord {
    pub country_code: String,
    pub country_name: String,
    pub gdp: u64,
    pub population: u64,
    pub gdp_per_capita: u64,
    pub gold: u32,
    pub silver: u32,
    pub bronze: u32,
    pub total: u32,
    pub gdp_per_medal: u64,
    pub gdp_per_capita_per_medal: u64,
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
        let country_code = fix_country_code(&country.country_code);

        let gdp = country_data.get(country_code).unwrap().gdp;
        let population = country_data.get(country_code).unwrap().population;
        let medals_record = MedalsRecord {
            country_code: country.country_code.clone(),
            country_name: country_data.get(country_code).unwrap().country_name.clone(),
            gdp,
            population,
            gdp_per_capita: gdp / population,
            gold: country.gold,
            silver: country.silver,
            bronze: country.bronze,
            total: country.total,
            gdp_per_medal: gdp / country.total as u64,
            gdp_per_capita_per_medal: (gdp / population) / country.total as u64,
        };
        medals_table.push(medals_record);
    }

    medals_table.sort_by(|country_1: &MedalsRecord, country_2: &MedalsRecord| {
        sort_by_gdp(country_1, country_2)
    });

    medals_table
}

fn sort_by_gdp(country_1: &MedalsRecord, country_2: &MedalsRecord) -> Ordering {
    match country_1.gdp_per_medal < country_2.gdp_per_medal {
        true => Ordering::Less,
        false => Ordering::Greater,
    }
}

// Fix-up the difference in coutry codes used by the World Bank and CBS
fn fix_country_code(code: &str) -> &str {
    match code {
        "GER" => "DEU",
        "RSA" => "ZAF",
        "ROM" => "ROU",
        "TKE" => "TUR",
        "FIJ" => "FJI",
        "KOS" => "XKX",
        "MGL" => "MNG",
        "CRO" => "HRV",
        "EIR" => "IRL",
        "SUI" => "CHE",
        "NED" => "NLD",
        "SLO" => "SVN",
        "GUA" => "GTM",
        _ => code,
    }
}
