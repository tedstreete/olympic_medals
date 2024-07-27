use regex::Regex;
use reqwest::blocking::Client;
use std::{error::Error, fs::File};

#[derive(Debug)]
struct Medals {
    country_name: String,
    gold: u32,
    silver: u32,
    bronze: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let medals = fetch_page(&client);
    Ok(())
}

fn fetch_page(client: &Client) -> Result<Vec<Medals>, Box<dyn Error>> {
    // let url = "https://www.cbssports.com/olympics/news/2024-paris-olympics-medal-count-tracker-for-how-many-gold-silver-bronze-medals-usa-each-country-has-won/";
    // let response = reqwest::blocking::get(url)?.text()?;
    // save_to_file("contents.txt", &response);
    let response = load_from_file("contents.txt")?;
    if response.len() > 1 {
        let medals: Vec<Medals> = parse(&response);
        return Ok(medals);
    }
    Err("Error".into())
    // let medals: Vec<Medals> = Vec::new();
    // Ok(medals)
}

fn parse(contents: &str) -> Vec<Medals> {
    // let re = Regex::new(r"(?P<frame><h2>2024 Paris Olympics medal count</h2>.*</table>)").unwrap();
    let re_frame =
        Regex::new(r"(?sm)(?P<frame><h2>2024 Paris Olympics medal count</h2>.*</table>)").unwrap();
    let frame_captures = re_frame.captures(contents).unwrap();
    let frame = &frame_captures["frame"];

    let re_table = Regex::new(r"(?sm)(?P<table><thead>.*</table>)").unwrap();
    let table_captures = re_table.captures(frame).unwrap();
    let table = &table_captures["table"];
    println!("{}", table);

    let medals: Vec<Medals> = Vec::new();

    medals
}

fn save_to_file(filename: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    std::io::Write::write_all(&mut file, content.as_bytes())?;
    Ok(())
}

// Function to load a string from a text file
fn load_from_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    std::io::Read::read_to_string(&mut file, &mut content)?;
    Ok(content)
}
