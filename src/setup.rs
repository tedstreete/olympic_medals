use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct WorldBankResponse {
    value: Option<f64>,
}

#[derive(Debug)]
struct CountryData {
    country_code: String,
    country_name: String,
    gdp: Option<f64>,
    population: Option<f64>,
}

fn fetch_world_bank_data(
    client: &Client,
    country_code: &str,
    indicator_code: &str,
) -> Result<Option<f64>, Box<dyn Error>> {
    let url = format!(
        "https://api.worldbank.org/v2/country/{}/indicator/{}?format=json",
        country_code, indicator_code
    );
    let response: Vec<serde_json::Value> = client.get(&url).send()?.json()?;

    if response.len() > 1 {
        if let Some(records) = response[1].as_array() {
            if let Some(latest_record) = records.first() {
                if let Some(value) = latest_record["value"].as_f64() {
                    return Ok(Some(value));
                }
            }
        }
    }
    Ok(None)
}

fn fetch_all_countries_data(
    client: &Client,
    countries: &Vec<(String, String)>,
) -> Result<Vec<CountryData>, Box<dyn Error>> {
    let gdp_indicator = "NY.GDP.MKTP.CD";
    let population_indicator = "SP.POP.TOTL";

    let mut country_data_list = Vec::new();

    for (country_code, country_name) in countries {
        let gdp = fetch_world_bank_data(client, &country_code, gdp_indicator)?;
        let population = fetch_world_bank_data(client, &country_code, population_indicator)?;

        println!(
            "{}-{}: {:?}, {:?}",
            country_code, country_name, gdp, population
        );

        country_data_list.push(CountryData {
            country_code: country_code.to_string(),
            country_name: country_name.to_string(),
            gdp,
            population,
        });
    }

    Ok(country_data_list)
}

fn write_to_csv(country_data_list: &[CountryData], file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(file_path)?;

    wtr.write_record(&["Country Code", "Country Name", "GDP", "Population"])?;

    for country_data in country_data_list {
        wtr.write_record(&[
            &country_data.country_code,
            &country_data.country_name,
            &country_data
                .gdp
                .map_or("N/A".to_string(), |v| v.to_string()),
            &country_data
                .population
                .map_or("N/A".to_string(), |v| v.to_string()),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Country {
    id: String,
    name: String,
    region: Region,
}

#[derive(Debug, Deserialize)]
struct Region {
    id: String,
    iso2code: String,
    value: String,
}

fn fetch_country_codes_and_names(client: &Client) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let url = "https://api.worldbank.org/v2/country?format=json&per_page=300";
    let response: Vec<serde_json::Value> = client.get(url).send()?.json()?;

    if response.len() > 1 {
        if let Some(countries) = response[1].as_array() {
            let country_data: Vec<(String, String)> = countries
                .iter()
                .filter_map(|country| {
                    let id = country["id"].as_str()?.to_string();
                    let name = country["name"].as_str()?.to_string();
                    Some((id, name))
                })
                .collect();
            return Ok(country_data);
        }
    }
    Ok(vec![])
}

fn setup() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let country_data = fetch_country_codes_and_names(&client)?;
    let country_data_list = fetch_all_countries_data(&client, &country_data)?;
    write_to_csv(&country_data_list, "country_data.csv")?;
    println!("CSV file 'country_data.csv' has been created with the GDP and population data.");
    Ok(())
}
