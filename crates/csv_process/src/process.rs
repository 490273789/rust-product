use std::fs;
use anyhow::Error;
use csv::Reader;
use serde::{Deserialize, Serialize};

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

pub fn process_csv(input: &str, output: &str) -> Result<(), Error> {
    let mut reader = Reader::from_path(input)?;
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
    fs::write(output, json)?;
    Ok(())
}
