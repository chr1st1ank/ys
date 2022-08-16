extern crate yaml_rust;
use std::fs::File;
use std::io;
use std::io::Error;
use std::process;

mod config;
mod output;

fn main() {
    let cfg = config::AppConfig::from_args();

    if let Err(e) = run(cfg) {
        error_exit(e);
    }
}

fn error_exit(e: Error) {
    eprintln!("Error opening file. {}", e);
    process::exit(1);
}

fn run(cfg: config::AppConfig) -> Result<(), io::Error> {
    let s = read_input(cfg.input.to_owned())?;
    let preprocessed_yaml = parse_yaml(s)?;

    output::print_to_stdout(preprocessed_yaml, cfg.should_colorize())?;

    Ok(())
}

fn read_input(file_path: Option<String>) -> Result<String, Error> {
    let mut reader: Box<dyn io::BufRead> = if let Some(fp) = file_path {
        let file_handle = File::open(fp)?;
        Box::new(io::BufReader::new(file_handle))
    } else {
        let file_handle = io::stdin().lock();
        Box::new(io::BufReader::new(file_handle))
    };
    let mut s = String::new();
    (*reader).read_to_string(&mut s)?;
    Ok(s)
}

fn parse_yaml(s: String) -> Result<String, io::Error> {
    let docs = yaml_rust::YamlLoader::load_from_str(s.as_str()).unwrap();

    // Dump the YAML object
    let mut out_str = String::new();
    for doc in docs.iter() {
        let mut emitter = yaml_rust::YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap();
        out_str.push('\n');
    }
    Ok(out_str)
}
