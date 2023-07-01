use clap::Parser;
use std::error::Error;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}



fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
    Ok(())

    // Getting user input 

     
}
// fn main() -> Result<(), Box<dyn Error>> {

//     let args = Cli::parse();

// }
// pub fn read_input_csv_file() -> Result<(), Box<dyn Error>> {
//     // Reader from path of test csv file
//     let reader = csv::Reader::from_path("sample_internet.csv");

//     // Iterate through each record (line) in the CSV
//     for record in reader?.records() {

//         let string_record = record?;

//         // Print the string record as an object
//         println!("{:?}", string_record);
//     }
//     Ok(())
// }

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}