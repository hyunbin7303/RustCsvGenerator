use clap:: {
    Args, Parser, Subcommand
};
// use std::error::Error;

#[derive(Debug,Parser)]
#[clap(author,version,about, long_about = None)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub userinput : EntityType,
    pub how_many_lines : usize
}

#[derive(Debug, Subcommand)]
pub enum EntityType {

    /// Set up cols and rows manually. 
    Custom(CustomCommand),

    /// Using Template that already existing.
    Template(TemplateCommand),

    /// Randomly Generate (Default 5 cols, 10 rows)
    AutoGenerate(AutoGenerateCommand),
}



#[derive(Debug, Args)]
pub struct CustomCommand {
    #[clap(subcommand)]
    pub command : CustomSubCommand,

}
#[derive(Debug, Args)]
pub struct TemplateCommand {
    #[clap(subcommand)]
    pub command : BuiltInSubCommand,
}
#[derive(Debug, Args)]
pub struct AutoGenerateCommand {
    #[clap(subcommand)]
    pub command : BuiltInSubCommand,
}

#[derive(Debug,Subcommand)]
pub enum BuiltInSubCommand {
    Create(CreateFile),
}

#[derive(Debug,Subcommand)]
pub enum CustomSubCommand {
    Create(CreateFile),
}
#[derive(Debug,Args)]
pub struct CreateFile {
    pub filename : String,
    pub cols : i64,
    // pub rows : i64, 
}
