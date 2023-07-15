use std::error::Error;
use std::io;
use csv::{ Writer, Reader };
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize)]
struct Customer {
    customer_guid: String,
    first_name: String,
    last_name: String,
    email: String,
    address: String,
}
pub fn create_sample_customer() -> Result<(), Box<dyn Error>> {
    // Creates new `Writer` for `stdout`
    let mut writer = csv::Writer::from_writer(io::stdout());

    // We don't explicitly write the header record
    writer.serialize(Customer {
        customer_guid: "6e49f2fc-00fd-4502-aed7-812da4aacbb8".to_string(),
        first_name: "Ailey".to_string(),
        last_name: "Benstead".to_string(),
        email: "abenstead0@state.gov".to_string(),
        address: "554 Mcguire Center".to_string(),
    })?;

    writer.serialize(Customer {
        customer_guid: "24349324-7e89-412e-b4bd-2a3c6d8e6d96".to_string(),
        first_name: "Ninnette".to_string(),
        last_name: "Wasmuth".to_string(),
        email: "nwasmuth1@washington.edu".to_string(),
        address: "10 Haas Circle".to_string(),
    })?;

    // A CSV writer maintains an internal buffer, so it's important
    // to flush the buffer when you're done.
    writer.flush()?;

    Ok(())
}



pub fn read_input_csv_file(filename: &str) -> Result<(), Box<dyn Error>> {
    // Reader from path of test csv file
    let reader = Reader::from_path(filename);

    // Iterate through each record (line) in the CSV
    for record in reader?.records() {

        let string_record = record?;

        // Print the string record as an object
        println!("{:?}", string_record);
    }
    Ok(())
}

// #[derive(Debug)] pub enum FileType { Csv, Json, Txt, NotDefined }
// pub fn evaluate_file_type(inputFile: &str) -> Option<FileType> {
//     match ".csv" => FileT
// }

pub fn create_sample_csv_file(filename: &str, col: i64, row: i64, headerRequired: bool) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    if headerRequired {
        wtr.write_record(&["Name", "Place", "ID"])?;
    }
    wtr.serialize(("Mark", "Sydney", 87))?;
    wtr.serialize(("Ashley", "Dublin", 32))?;
    wtr.serialize(("Akshat", "Delhi", 11))?;
    wtr.flush()?;
    Ok(())
}

pub fn inspect_testing(input: &String, digit: bool) -> (i32, String) {
    if !digit {
        return (input.len() as i32, String::from("char"));
    }
    return (inspect_numbers(input), String::from("digit"));
}

fn inspect_numbers(input: &String) -> i32 {
    let mut count = 0;
    for c in input.chars() {
        if c.is_digit(10) {
            count += 1;
        }
    }
    return count;
}