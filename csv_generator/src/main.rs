use clap::Parser;
use std::io;
use std::error::Error;
mod args;
mod file_handler;
mod commands;
fn main() -> Result<(), Box<dyn Error>> {
    let args: args::CliArgs = args::CliArgs::parse(); 
    match &args.userinput {
        Some(commands::ActionType::AutoGenerate(cmd)) => {
            println!("Option for autogenerationg csv file.");
            println!("1. Create default random csv file(Default : 5 columns, 5 rows");
            println!("2. Set up the number of cols and rows");
            let mut numChosen = String::new();
            io::stdin().read_line(&mut numChosen).expect("Failed to readline");
            if numChosen.trim() == "1" {
                let samplename: &str = "Testing.csv";
                let result = file_handler::create_sample_customer();
            }
            else if numChosen.trim() == "2" {
                println!("Please enter column and row data:");
                let mut input_col = String::new();
                let mut input_row = String::new();
                io::stdin().read_line(&mut input_col).expect("Not a valid string");
                let col: i64 = input_col.trim().parse().expect("Not a valid number");
                io::stdin().read_line(&mut input_row).expect("Not a valid string");
                let row: i64 = input_row.trim().parse().expect("Not a valid number");
                // Todo : File name should be the datetime
                let result = file_handler::create_sample_csv_file("AutoGenerate.csv", col, row, true);
                println!("col:{},Row:{}", col,row);
            }
            else {
                println!("Invalid input. Please press button 1 or 2");
            }
        }
        Some(commands::ActionType::Custom(cmd)) => {
            // match cmd.
        }
        Some(commands::ActionType::Template(cmd)) =>{
            match cmd.filepath {
                Some(ref _fileName) => {
                    println!("File Name : {}", _fileName);
                    let test = file_handler::read_input_csv_file(_fileName);

                }
                None => {
                    println!("Input is not valid");
                }
            }
        }
        None => {
            println!("Not valid input");
        }
    }
   // match &arg.command {
    // Commands::Caption(options) => {
        // generate_caption(options)
    Ok(())
}



