use clap::Parser;
use std::error::Error;
mod args;


fn main() -> Result<(), Box<dyn Error>> {
    let args: args::CliArgs = args::CliArgs::parse(); 
    println!("{:?}", args);
    println!("{}", args.how_many_lines);
   // match &arg.command {
    // Commands::Caption(options) => {
        // generate_caption(options)
    //}
   //}

    
    // Get how many 

    Ok(())
}
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



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_input_csv_file_should_returnOk()
    {
    //     assert_eq!(add(1, 2), 3);
    }
}