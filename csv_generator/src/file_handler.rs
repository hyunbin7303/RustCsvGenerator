use std::error::Error;

pub fn read_input_csv_file() -> Result<(), Box<dyn Error>> {
    // Reader from path of test csv file
    let reader = csv::Reader::from_path("sample_internet.csv");

    // Iterate through each record (line) in the CSV
    for record in reader?.records() {

        let string_record = record?;

        // Print the string record as an object
        println!("{:?}", string_record);
    }
    Ok(())
}
