use clap::Parser;
use std::fs::File;
use std::io;
use std::io::{BufRead, Error};
use std::process;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct CommandLineArgs {
    /// Input file or "--" to read from stdin
    input: Option<String>,
}

fn main() {
    let args = CommandLineArgs::parse();
    let file_path = args.input;

    if let Err(e) = process_stream(file_path) {
        error_exit(e);
    }
}

fn error_exit(e: Error) {
    eprintln!("Error opening file. {}", e);
    process::exit(1);
}

fn process_stream(file_path: Option<String>) -> Result<(), io::Error> {
    let reader = open_input_stream(file_path)?;
    write_out(reader)?;
    Ok(())
}

fn open_input_stream(file_path: Option<String>) -> Result<Box<dyn io::BufRead>, io::Error> {
    if let Some(fp) = file_path {
        let file_handle = File::open(fp)?;
        Ok(Box::new(io::BufReader::new(file_handle)))
    } else {
        let file_handle = io::stdin().lock();
        Ok(Box::new(io::BufReader::new(file_handle)))
    }
}

fn write_out(stream: Box<dyn io::BufRead>) -> Result<(), io::Error> {
    for line in stream.lines() {
        println!("{}", line?);
    }
    Ok(())
}
