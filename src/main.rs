use std::error::Error;
use std::fs::File;
use csv::Writer;
use scraper::{Html, Selector};
use reqwest::blocking::Client;

const URL: &str = "https://www.ercot.com/content/cdr/html/20230213_dam_spp.html";


fn main() -> Result<(), Box<dyn Error>> {
    println!("Start");

    // Get Response from ERCOT
    let client = Client::new();
    let response = client.get(URL)
        .send()?
        .text()?;

      //Parse the data
    let html = Html::parse_document(&response);
    

    println!("HTML:\n{}", response);

    //Prepare the output

    //Write the data to csv file
    let file = File::create("output.csv")?;
    let mut writer = Writer::from_writer(file);
    
    
    writer.write_record(&["Column 1", "Column 2", "Value 1", "Value 2"])?; //test 

    writer.flush()?;
    
    println!("End");
    Ok(())
}