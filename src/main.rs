use std::error::Error;
use std::fs::File;
use csv::Writer;
use scraper::{Html, Selector};
use reqwest::blocking::Client;
use serde::Deserialize;

const URL: &str = "https://www.ercot.com/content/cdr/html/20230213_dam_spp.html";


#[derive(Debug, PartialEq)]
struct ErcotData {
    lz_houston: String,
    lz_south: String,
    lz_north: String,
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
    let mut_lz_west = String::new();
    
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

        println!("{}", lz_houston_string); //test
        }

        let mut lz_south_string = String::new();
        if let Some(td) = lz_south {
         lz_south_string = td.text().collect::<String>();

        println!("{}", lz_south_string); //test
        }

        let mut lz_north_string = String::new();
        if let Some(td) = lz_north {
         lz_north_string = td.text().collect::<String>();

        println!("{}", lz_north_string); //test
        }

        let mut lz_west_string = String::new();
        if let Some(td) = lz_west {
          lz_west_string = td.text().collect::<String>();

        println!("{}", lz_west_string); //test
      }  
     }

    

                 //LZ_HOUSTON Header

    let houston_header = Selector::parse("th.headerValueClass").unwrap();  
    let element = document.select(&houston_header)
      .find(|th| th.text().collect::<String>() == "LZ_HOUSTON");

    println!("{:?}", element.unwrap().text().collect::<String>()); //test 

                 //LZ_SOUTH Header

    let south_selector = Selector::parse("th.headerValueClass").unwrap();
    let element = document.select(&south_selector)
    .find(|th| th.text().collect::<String>() == "LZ_SOUTH");

    println!("{:?}", element.unwrap().text().collect::<String>()); //test 
     

                //LZ_NORTH Header

    let north_selector = Selector::parse("th.headerValueClass").unwrap();
    let element = document.select(&north_selector)
    .find(|th| th.text().collect::<String>() == "LZ_NORTH");

    println!("{:?}", element.unwrap().text().collect::<String>()); //test 
     
                
                 //LZ_WEST Header

    let west_selector = Selector::parse("th.headerValueClass").unwrap();
    let element = document.select(&west_selector)
    .find(|th| th.text().collect::<String>() == "LZ_WEST");

    println!("{:?}", element.unwrap().text().collect::<String>()); //test 
             
    //Store data

    //Prepare the output

    //Write the data to csv file
    let file = File::create("output.csv")?;
    let mut writer = Writer::from_writer(file);
    
    
    writer.write_record(&["Column 1", "Column 2", "Value 1", "Value 2"])?; //test 

    writer.flush()?;
    
    println!("End");
    Ok(())
}