use std::fs::metadata;
use chrono::{prelude::DateTime, Local};
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// The file path of the file to check the creation datetime.
    #[arg(value_name = "FILEPATH")]
    filename: String,
}

fn main() {
    let args = Args::parse();
    let created_timestamp = match metadata(args.filename) {
        Ok(metadata) => match metadata.created() {
            Ok(system_time) => system_time,
            Err(e) => return eprintln!("{e}")
        },
        Err(e) => return eprintln!("{e}")
    };

    let local_time: DateTime<Local> = created_timestamp.into();
    println!("{}", local_time.format("%Y/%m/%d %T"));
}
