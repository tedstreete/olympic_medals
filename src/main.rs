mod country_data;
mod medals;
mod table;

use std::error::Error;

use country_data::CountryData;
use csv::Writer;
use medals::Medals;
use table::MedalsRecord;

fn main() -> Result<(), Box<dyn Error>> {
    let country_data = CountryData::get_country_data().unwrap();
    let medals = Medals::fetch_medals().unwrap();
    let table = table::assemble_table(country_data, medals);
    save_to_csv(table, "Medal Records.csv")?;

    Ok(())
}

fn save_to_csv(records: Vec<MedalsRecord>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(file_path)?;

    for record in records {
        wtr.serialize(record)?;
    }

    wtr.flush()?;
    Ok(())
}
