use std::error::Error;
use std::fs::File;
use csv::Writer;
use scraper::{Html, Selector};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

const URL: &str = "https://www.ercot.com/content/cdr/html/20230213_dam_spp.html";


#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ErcotData {
    #[serde(rename = "LZ_HOUSTON")]   // rename the fields to match html data
    lz_houston: String,
    #[serde(rename = "LZ_SOUTH")]
    lz_south: String,
    #[serde(rename = "LZ_NORTH")]
    lz_north: String,
    #[serde(rename = "LZ_WEST")]
    lz_west: String,
}


fn main() -> Result<(), Box<dyn Error>> {
    println!("Start");

     // Get Response from ERCOT
    let client = Client::new();
    let response = client.get(URL)
        .send()?
        .text()?;
    
    //Parse the data
    let document = scraper::Html::parse_document(&response);

    // Initialize variables to be looped through
    let mut lz_houston = String::new(); 
    let mut lz_south = String::new();
    let mut lz_north = String::new();
    let mut lz_west = String::new();
    
    
    let mut ercot_data = Vec::new();

    // Iterate thorugh "td" elements to collect lz data points
    for row in document.select(&scraper::Selector::parse("tr").unwrap()) {
        let lz_houston = row.select(&scraper::Selector::parse("td:nth-child(12)").unwrap()).next(); 
        let lz_south = row.select(&scraper::Selector::parse("td:nth-child(16)").unwrap()).next();  
        let lz_north = row.select(&scraper::Selector::parse("td:nth-child(14)").unwrap()).next(); 
        let lz_west = row.select(&scraper::Selector::parse("td:nth-child(17)").unwrap()).next(); 
        
        //create new variable to store data as string
        
        let mut lz_houston_string = String::new();   
        if let Some(td) = lz_houston {
         lz_houston_string = td.text().collect::<String>();
        }

        let mut lz_south_string = String::new();
        if let Some(td) = lz_south {
         lz_south_string = td.text().collect::<String>();
        }

        let mut lz_north_string = String::new();
        if let Some(td) = lz_north {
         lz_north_string = td.text().collect::<String>();
        }

        let mut lz_west_string = String::new();
        if let Some(td) = lz_west {
          lz_west_string = td.text().collect::<String>();
        }  
     
        ercot_data.push(ErcotData { 
            lz_houston: lz_houston_string, 
            lz_south: lz_south_string, 
            lz_north: lz_north_string, 
            lz_west: lz_west_string,
         });
     
      }

      
    // Write the data to a CSV file
   
    let file = File::create("output.csv")?;
    let mut writer = Writer::from_writer(file);

    // Write the data rows
    for data in ercot_data {
    writer.serialize(data)?;
    }

    writer.flush()?;

    println!("end");
    Ok(())
}