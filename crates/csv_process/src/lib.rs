mod r_cli;
pub use r_cli::{CsvOption, Opts, OutputFormat, SubCommand};

mod process;
pub use process::process_csv;
