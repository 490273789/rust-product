use anyhow::Error;
use csv::Reader;
// use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use crate::OutputFormat;

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// struct Record {
//     name: String,
//     position: String,
//     #[serde(rename = "DOB")]
//     dob: String,
//     nationality: String,
//     #[serde(rename = "Kit Number")]
//     kit_number: u8,
// }

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<(), Error> {
    let mut reader = Reader::from_path(input)?;
    // let records = reader
    //     .deserialize()
    //     .map(|record| record?)
    //     .collect::<Vec<Record>>();
    // println!("records is ：{records:?}");
    let mut res = Vec::with_capacity(128);
    // 读取headers
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        // anyhow的使用 -> ?, 相当于一下代码
        // match result {
        //     Ok(res) => res,
        //     Err(e) => Err(e.into)
        // }
        // 组装数据
        let json_data = headers.iter().zip(record.iter()).collect::<Value>();
        res.push(json_data);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&res)?,
        OutputFormat::Yaml => serde_yaml::to_string(&res)?,
    };

    fs::write(output, content)?;
    Ok(())
}
