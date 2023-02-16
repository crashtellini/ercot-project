use std::error::Error;
use std::fs::File;
use csv::Writer;
use scraper::{Html, Selector};
use reqwest::blocking::Client;
use serde::Deserialize;

const URL: &str = "https://www.ercot.com/content/cdr/html/20230213_dam_spp.html";


#[derive(Debug, PartialEq)]
struct ErcotData {
    lz_houston: f32,
    lz_south: f32,
    lz_north: f32,
    lz_west: f32,
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

    // iterate over each `tr` element
    for row in document.select(&scraper::Selector::parse("tr").unwrap()) {
    let lz_houston = row.select(&scraper::Selector::parse("td:nth-child(12)").unwrap()).next();
    let lz_south = row.select(&scraper::Selector::parse("td:nth-child(16)").unwrap()).next();
    let lz_north = row.select(&scraper::Selector::parse("td:nth-child(14)").unwrap()).next();
    let lz_west = row.select(&scraper::Selector::parse("td:nth-child(17)").unwrap()).next();
    
    // test
    if let Some(td) = lz_houston {
    println!("{}", td.text().collect::<String>()); //test
    }
    }    

                 //LZ_HOUSTON Header

    let selector = Selector::parse("th.headerValueClass").unwrap();  
    let element = document.select(&selector)
      .find(|th| th.text().collect::<String>() == "LZ_HOUSTON");

    println!("{:?}", element.unwrap().text().collect::<String>()); //test 

                 //LZ_SOUTH Header

    let selector = Selector::parse("th.headerValueClass").unwrap();
    let element = document.select(&selector)
    .find(|th| th.text().collect::<String>() == "LZ_SOUTH");

    println!("{:?}", element.unwrap().text().collect::<String>()); //test 
     

                //LZ_NORTH Header

    let selector = Selector::parse("th.headerValueClass").unwrap();
    let element = document.select(&selector)
    .find(|th| th.text().collect::<String>() == "LZ_NORTH");

    println!("{:?}", element.unwrap().text().collect::<String>()); //test 
     
                
                 //LZ_WEST Header

    let selector = Selector::parse("th.headerValueClass").unwrap();
    let element = document.select(&selector)
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