use std::path::Path;

use clap::{command, Parser};

#[derive(Debug, Parser)]
#[command(name = "r_cli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats.")]
    Csv(CsvOption),
}

#[derive(Parser, Debug)]
pub struct CsvOption {
    #[arg(short, long, value_parser=verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    pub output: String,
    #[arg(short, long, default_value_t = ',')] // default_value_t 不会做自动转换
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
