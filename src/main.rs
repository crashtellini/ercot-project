#![allow(warnings)]
use chrono::{NaiveTime, TimeZone, Utc};
use csv::Writer;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::{error::Error, thread::current};

const ERCOT_BASE_URL: &str = "https://www.ercot.com/content/cdr/html/";
const CSV_FILE_NAME: &str = "ercot_data.csv";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ErcotData {
    #[serde(rename = "LZ_HOUSTON")] // rename the fields to match html data
    lz_houston: String,
    #[serde(rename = "LZ_SOUTH")]
    lz_south: String,
    #[serde(rename = "LZ_NORTH")]
    lz_north: String,
    #[serde(rename = "LZ_WEST")]
    lz_west: String,
}
fn main() -> Result<(), Box<dyn Error>> {
    println!("Initializing Ercot HTML Scraper");

    // Get Request
    let client = Client::new();
    let response = client.get(ercot_dynamic_url()).send()?.text()?;

    //Parse Response
    let document = scraper::Html::parse_document(&response);

    let mut ercot_data = Vec::new();

    // Iterate thorugh "td" elements to collect lz data points
    for row in document.select(&scraper::Selector::parse("tr").unwrap()) {
        let lz_houston = select_cell("td:nth-child(12)", &row); //returns theth element of html
        let lz_south = select_cell("td:nth-child(16)", &row);
        let lz_north = select_cell("td:nth-child(14)", &row);
        let lz_west = select_cell("td:nth-child(17)", &row);

        // create new variable to store data as string
        let lz_houston_string = extract_text(lz_houston);
        let lz_south_string = extract_text(lz_south);
        let lz_north_string = extract_text(lz_north);
        let lz_west_string = extract_text(lz_west);

        // If all data points are empty strings, skip the row
        if lz_houston_string.is_empty()
            && lz_south_string.is_empty()
            && lz_north_string.is_empty()
            && lz_west_string.is_empty()
        {
            continue;
        }

        ercot_data.push(ErcotData {
            lz_houston: lz_houston_string,
            lz_south: lz_south_string,
            lz_north: lz_north_string,
            lz_west: lz_west_string,
        });
    }

    // Write the data to a CSV file
    let file_path = CSV_FILE_NAME;
    write_to_csv(&ercot_data, file_path).expect("Failed to write to CSV");

    println!("end");
    Ok(())
}

// dynamically construct the url based on if the current time is before or after 20:00 UTC
fn ercot_dynamic_url() -> String {
    let now = Utc::now();
    let market_open_time = Utc.from_utc_datetime(
        &now.naive_utc()
            .date()
            .and_time(NaiveTime::from_hms_opt(20, 0, 0).unwrap()),
    );
    let current_date = if now >= market_open_time {
        now.naive_utc().date().succ().format("%Y%m%d").to_string()
    } else {
        now.naive_utc().date().format("%Y%m%d").to_string()
    };
    format!("{}{}_dam_spp.html", ERCOT_BASE_URL, current_date)
}

fn extract_text(td: Option<scraper::ElementRef>) -> String {
    if let Some(td) = td {
        td.text().collect::<String>()
    } else {
        String::new()
    }
}

fn select_cell<'a>(
    selector: &'a str,
    row: &'a scraper::ElementRef<'a>,
) -> Option<scraper::ElementRef<'a>> {
    row.select(&scraper::Selector::parse(selector).unwrap())
        .next()
}

fn write_to_csv<T: Serialize>(data: &[T], file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(file_path)?;
    let mut writer = Writer::from_writer(file);

    for data_row in data {
        writer.serialize(data_row)?;
    }

    writer.flush()?;
    Ok(())
}

//checks if the URL generated matches the expected format
//It also checks if the length of the generated URL matches the length of the expected format string
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ercot_dynamic_url() {
        let url = ercot_dynamic_url();
        let expected_url_format = "https://www.ercot.com/content/cdr/html/YYYYMMDD_dam_spp.html";
        assert!(
            url.starts_with("https://www.ercot.com/content/cdr/html/"),
            "URL format is incorrect"
        );
        assert!(url.ends_with("_dam_spp.html"), "URL format is incorrect");
        assert_eq!(
            url.len(),
            expected_url_format.len(),
            "URL format is incorrect"
        );
    }
}
