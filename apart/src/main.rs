

extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process;


#[derive(Debug)]
struct Person {
    name: String,
    last_time_seen: String,
}

fn read_csv() -> Result<(), Box<Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::Reader::from_path(file_path)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = read_csv() {
        println!("{}", err);
        process::exit(1);
    }
}
