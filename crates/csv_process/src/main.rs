use clap::Parser;
use csv_process::{process_csv, Opts, SubCommand};

// r_cli csv -i input.csv -o output.json --header -d

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{opts:?}");
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            println!("output: {output}");
            process_csv(&opts.input, output, opts.format)?
        }
    }
    Ok(())
}
