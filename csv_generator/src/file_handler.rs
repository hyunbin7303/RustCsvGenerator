use std::error::Error;
use csv::{ Writer, Reader };
pub fn read_input_csv_file() -> Result<(), Box<dyn Error>> {
    // Reader from path of test csv file
    let reader = Reader::from_path("sample_internet.csv");

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

pub fn create_sample_csv_file(filename: &str, col: i64, row: i64) -> Result<(), Box<dyn Error>> {
    println!("File Generate : {}", filename);
    let mut wtr = Writer::from_path(filename)?;
    wtr.write_record(&["a", "b", "c"])?;
    wtr.write_record(&["x", "y", "z"])?;
    wtr.flush()?;

    Ok(())
}
