use regex::Regex;
use scraper::{Html, Selector};
use std::{
    error::Error,
    fs::{self, File},
    path::Path,
    time::{Duration, SystemTime},
};

const CACHE: &str = "contents.txt";

#[derive(Debug)]
pub(crate) struct Medals {
    pub country_code: String,
    pub country_name: String,
    pub gold: u32,
    pub silver: u32,
    pub bronze: u32,
    pub total: u32,
}

impl Medals {
    pub fn fetch_medals() -> Result<Vec<Medals>, Box<dyn Error>> {
        if update_records(CACHE) {
            let url = "https://www.cbssports.com/olympics/news/2024-paris-olympics-medal-count-tracker-for-how-many-gold-silver-bronze-medals-usa-each-country-has-won/";
            let response = reqwest::blocking::get(url)?.text()?;
            save_to_file("contents.txt", &response)?;
        }
        let mut response = load_from_file(CACHE)?;

        response.insert_str(0, "<table>");
        if response.len() > 1 {
            let medals: Vec<Medals> = Medals::extract_table(&response);
            return Ok(medals);
        }
        Err("Error".into())
    }

    fn extract_table(contents: &str) -> Vec<Medals> {
        // Ignore superflous html prior to the medals table
        let re_frame =
            Regex::new(r"(?sm)(?P<frame><h2>\s*2024 Paris Olympics medal count\s*</h2>.*</table>)")
                .unwrap();
        let frame_captures = re_frame.captures(contents).unwrap();
        let frame = &frame_captures["frame"];

        let re_table = Regex::new(r"(?sm)(?P<table><thead>.*</table>)").unwrap();
        let table_captures = re_table.captures(frame).unwrap();
        let table_proto = &table_captures["table"];
        let mut table = String::from("<table>");
        table.push_str(table_proto);

        let medals = Medals::parse_table(&table);

        medals
    }

    fn parse_table(fragment_source: &str) -> Vec<Medals> {
        let mut medals: Vec<Medals> = Vec::new();

        let tbody_selector = Selector::parse("tbody").unwrap();
        let row_selector = Selector::parse("tr").unwrap();
        let cell_selector = Selector::parse("td").unwrap();

        let fragment = Html::parse_fragment(fragment_source);
        let tbody = fragment.select(&tbody_selector).next().unwrap();
        for row in tbody.select(&row_selector) {
            let mut cells = row.select(&cell_selector);
            let country = cells.next().unwrap().text().collect::<Vec<_>>().concat();
            let country = remove_whitespace(&country);
            let country_name = country[3..].to_string();
            let country_code = country[..3].to_string();
            let gold = cells.next().unwrap().text().collect::<Vec<_>>().concat();
            let gold: u32 = remove_whitespace(&gold).parse().unwrap();
            let silver = cells.next().unwrap().text().collect::<Vec<_>>().concat();
            let silver: u32 = remove_whitespace(&silver).parse().unwrap();
            let bronze = cells.next().unwrap().text().collect::<Vec<_>>().concat();
            let bronze: u32 = remove_whitespace(&bronze).parse().unwrap();
            let total = cells.next().unwrap().text().collect::<Vec<_>>().concat();
            let total: u32 = remove_whitespace(&total).parse().unwrap();
            let medals_for_country = Medals {
                country_code,
                country_name,
                gold,
                silver,
                bronze,
                total,
            };
            medals.push(medals_for_country);
        }
        medals
    }
}

fn remove_whitespace(input: &str) -> String {
    input.chars().filter(|c| !c.is_whitespace()).collect()
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

// Don't update the records more frequently than every 60 minutes
fn update_records<P: AsRef<Path>>(path: P) -> bool {
    let now = SystemTime::now();
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => return true,
    };

    if let Ok(modified_time) = metadata.modified() {
        if let Ok(duration_since_modified) = now.duration_since(modified_time) {
            return duration_since_modified >= Duration::from_secs(60 * 60);
        }
    }

    false
}
