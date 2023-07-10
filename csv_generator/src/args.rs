use clap:: { Args, Parser, Subcommand };
use crate::commands::ActionType;
#[derive(Debug,Parser)]
#[clap(author,version,about, long_about = None)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub userinput : Option<ActionType>,
    // pub how_many_lines : usize,

    // #[clap(global = false, long)]
    // json: bool,
}




