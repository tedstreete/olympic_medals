use csv::Reader;
use std::collections::HashMap;
use std::error::Error;

pub struct CountryData {
    pub country_name: String,
    pub gdp: u64,
    pub population: u64,
}

impl CountryData {
    pub fn get_country_data() -> Result<HashMap<String, CountryData>, Box<dyn Error>> {
        let file_path = "country_data.csv";
        let mut rdr = Reader::from_path(file_path)?;
        let mut country_data = HashMap::new();

        for record in rdr.records() {
            let record = record?;
            if record.len() < 4 {
                return Err(From::from("Country Data file has less than 4 columns"));
            }
            let key = record[0].to_string();
            let country_name = record[1].to_string();
            let gdp = record[2].parse::<f64>().map_or_else(|_| 1, |x| x as u64);
            let population = record[3].parse::<u64>().map_or_else(|_| 1, |x| x);
            country_data.insert(
                key,
                CountryData {
                    country_name,
                    gdp,
                    population,
                },
            );
        }

        Ok(country_data)
    }
}
