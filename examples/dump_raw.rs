use std::fs::File;
use std::env;
use std::io::{self, BufReader};
use hangouts_json_parser::Hangouts;

fn main() -> Result<(), io::Error> {
    let input = if let Some(path) = env::args_os().nth(1) {
        BufReader::new(File::open(path)?)
    } else {
        eprintln!("usage: {} <path>", env::args().nth(0).unwrap());
        std::process::exit(1);
    };

    let h: Hangouts = serde_json::from_reader(input)?;

    println!("{:#?}", h);
    Ok(())
}
