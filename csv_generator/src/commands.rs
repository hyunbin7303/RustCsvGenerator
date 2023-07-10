use clap:: { Args, Parser, Subcommand };
#[derive(Debug, Subcommand)]
pub enum ActionType {

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
    pub filepath: Option<String>,
}
#[derive(Debug, Args)]
pub struct AutoGenerateCommand {
    // pub string: Option<String>,

    // #[arg(short = 'd', long = "digits")]
    // pub col_num: i64, 
    // pub need_digit: bool

    // #[arg(short = 'h', long = "header")]
    // pub need_header: bool,

}
#[derive(Debug,Subcommand)]
pub enum CustomSubCommand {
    Create(CreateFile),
}
#[derive(Debug,Args)]
pub struct CreateFile {
    pub filename : String,
    pub cols : i64,
}
    
