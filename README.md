# ercot-project

This Rust program is a simple HTML scraper that extracts data from the ERCOT (Electric Reliability Council of Texas) website and saves it to a CSV file.

### Prerequisites 

Rust 

### Installation 

* Clone the repository to your local machine.
* Navigate to the project directory in your terminal.

### Running the program
To run the program, execute the following command:
``` 
cargo run 
``` 
### Running the test 
To run the tests, execute the following command:
```
cargo test 
```
### Usage 

* Once you run the program, it will fetch the data from the ERCOT website and save it to a CSV file named ercot_data.csv.
* The program will check if the current time is before or after 20:00 UTC and dynamically generate the URL to fetch the data accordingly. 
* The data is collected from the td elements of the HTML table and stored in a vector of ErcotData structs. 
* The write_to_csv function serializes the data in the vector and writes it to a CSV file using the csv crate.
