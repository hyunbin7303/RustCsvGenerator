use clap::Parser;
use std::error::Error;
use crate::args::{ActionType};
mod args;
mod file_handler;

fn main() -> Result<(), Box<dyn Error>> {
    let args: args::CliArgs = args::CliArgs::parse(); 
    match &args.userinput {
        Some(ActionType::AutoGenerate(cmd)) => {
            match cmd.string {
                Some(ref _name)=> {
                    let test = file_handler::read_input_csv_file();
                }
                None => { 
                }

            }
        }
        Some(ActionType::Custom(cmd)) => {
            // match cmd.
        }
        Some(ActionType::Template(cmd)) =>{
            match cmd.filepath {
                Some(ref _name) => {
                    // let 
                }
                None => {

                }
            }
        }
        None => {
            println!("Not valid input");
        }
    }
    // println!("{:?}", args);
    println!("{}", args.how_many_lines);
   // match &arg.command {
    // Commands::Caption(options) => {
        // generate_caption(options)
    //}
   //}
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