use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let file_handle = File::open(file_path);
    if let Err(e) = file_handle {
        eprintln!("Error opening file. {}", e);
        process::exit(1);
    }
    let reader = BufReader::new(file_handle.unwrap());
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
