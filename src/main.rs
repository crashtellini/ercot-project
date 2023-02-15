use std::error::Error;
use std::fs::File;
use csv::Writer;
use scraper::{Html, Selector};
use reqwest::blocking::Client;
use serde::Deserialize;

const URL: &str = "https://www.ercot.com/content/cdr/html/20230213_dam_spp.html";


#[derive(Debug, PartialEq)]
struct Data {
    oper_day: String,
    hour_ending: i32,
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

      //Select the data table using a CSS selector

      let selector = Selector::parse("th.headerValueClass").unwrap();
      
       //Return only the desired element "i.e. LZ_HOUSTON"
      let element = document.select(&selector)
      .find(|th| th.text().collect::<String>() == "LZ_HOUSTON");

      

      println!("{:?}", element.unwrap().text().collect::<String>()); //test 
      
      // iterate over each `tr` element
      for row in document.select(&scraper::Selector::parse("tr").unwrap()) {
      // extract the twelfth `td` element using the `nth` function
      let twelfth_td = row.select(&scraper::Selector::parse("td:nth-child(12)").unwrap()).next();
    
      // print the text content of the 12th `td` element
      if let Some(td) = twelfth_td {
        println!("{}", td.text().collect::<String>());
      }
      }

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