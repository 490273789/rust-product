use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

// r_cli csv -i input.csv -o output.json --header -d
#[derive(Debug, Parser)]
#[command(name = "r_cli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats.")]
    Csv(CsvOption),
}

#[derive(Parser, Debug)]
struct CsvOption {
    #[arg(short, long, value_parser=verify_input_file)]
    input: String,
    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    output: String,
    #[arg(short, long, default_value_t = ',')] // default_value_t 不会做自动转换
    delimiter: char,
    #[arg(long, default_value_t = true)]
    header: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: u8,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{opts:?}");
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;
            // let records = reader
            //     .deserialize()
            //     .map(|record| record?)
            //     .collect::<Vec<Record>>();
            // println!("records is ：{records:?}");
            let mut res = Vec::with_capacity(128);
            for result in reader.deserialize() {
                let record: Record = result?;
                // anyhow的使用 -> ?, 相当于一下代码
                // match result {
                //     Ok(res) => res,
                //     Err(e) => Err(e.into)
                // }
                res.push(record);
            }
            let json = serde_json::to_string_pretty(&res)?;
            fs::write(opts.output, json)?;
        }
    }
    Ok(())
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
