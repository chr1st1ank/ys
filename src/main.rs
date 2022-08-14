use std::{env, process};
use std::fs::File;
use std::io;
use std::io::{BufRead, Error};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    if let Err(e) = process_stream(file_path){
        error_exit(e);
    }
}

fn error_exit(e: Error) {
    eprintln!("Error opening file. {}", e);
    process::exit(1);
}

fn process_stream(file_path: &String) -> Result<(), io::Error> {
    let reader = open_input_stream(file_path)?;
    write_out(reader)?;
    Ok(())
}

fn open_input_stream(file_path: &String) -> Result<io::BufReader<File>, io::Error> {
    let file_handle = File::open(file_path)?;
    let reader = io::BufReader::new(file_handle);
    Ok(reader)
}

fn write_out<T>(stream: io::BufReader<T>) -> Result<(), io::Error>
where
    T: std::io::Read,
{
    for line in stream.lines() {
        println!("{}", line?);
    }
    Ok(())
}
