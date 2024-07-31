mod country_data;
mod medals;
mod table;

use num_format::{Locale, ToFormattedString};
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

use country_data::CountryData;
use csv::Writer;
use medals::Medals;
use table::MedalsRecord;

fn main() -> Result<(), Box<dyn Error>> {
    let country_data = CountryData::get_country_data().unwrap();
    let medals = Medals::fetch_medals().unwrap();
    let table_data = table::assemble_table(country_data, medals);
    let html_table = get_html_table(&table_data);
    create_index_html(&html_table)?;
    save_to_csv(&table_data, "Medal-Records.csv")?;

    Ok(())
}

fn get_html_table(table_data: &Vec<MedalsRecord>) -> String {
    let mut table = String::new();
    for country_medals in table_data {
        // table.push_str("    <tr>\n");
        // table.push_str(&format!("      <td>{}</td>", country_medals.country_code));
        table.push_str(&format!("      <td>{}</td>", country_medals.country_name));
        table.push_str(&format!("      <td>{}</td>", dollar(country_medals.gdp)));
        table.push_str(&format!(
            "      <td>{}</td>",
            comma_delim(country_medals.population)
        ));
        table.push_str(&format!(
            "      <td>{}</td>",
            dollar(country_medals.gdp_per_capita)
        ));
        table.push_str(&format!(
            "      <td>{}</td>",
            comma_delim(country_medals.gold)
        ));
        table.push_str(&format!(
            "      <td>{}</td>",
            comma_delim(country_medals.silver)
        ));
        table.push_str(&format!(
            "      <td>{}</td>",
            comma_delim(country_medals.bronze)
        ));
        table.push_str(&format!(
            "      <td>{}</td>",
            comma_delim(country_medals.total)
        ));
        table.push_str(&format!(
            "      <td>{}</td>",
            dollar(country_medals.gdp_per_medal)
        ));
        table.push_str(&format!(
            "      <td>{}</td>",
            dollar(country_medals.gdp_per_capita_per_medal)
        ));
        table.push_str("    </tr>\n");
    }
    table
}

fn create_index_html(replacement: &str) -> Result<(), Box<dyn Error>> {
    // Read the file content
    let mut content = String::new();
    File::open("template.html")?.read_to_string(&mut content)?;
    let modified_content = content.replace("[template]", replacement);

    // Write the modified content back to the file
    let mut file = File::create("index.html")?;
    file.write_all(modified_content.as_bytes())?;

    Ok(())
}

fn save_to_csv(records: &Vec<MedalsRecord>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(file_path)?;

    for record in records {
        wtr.serialize(record)?;
    }

    wtr.flush()?;
    Ok(())
}

fn comma_delim<T: Into<u64>>(num: T) -> String {
    let num_u64: u64 = num.into();
    num_u64.to_formatted_string(&Locale::en)
}

fn dollar<T: Into<u64>>(num: T) -> String {
    let num_u64: u64 = num.into();
    let mut currency = String::from("$");
    let comma_format = comma_delim(num_u64);
    currency.push_str(&comma_format);
    currency
}
