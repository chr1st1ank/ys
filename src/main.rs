extern crate yaml_rust;
use std::fs::File;
use std::io::Error;
use std::process;
use std::io;

mod branch_selection;
mod config;
mod output;
mod yaml_parsing;

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
    println!("{}", "-".repeat(20));
    output::print_to_stdout(&s, true)?;
    println!("{}", "-".repeat(20));

    let yaml_docs_filtered = filter_yaml_docs(parse_yaml(s), &cfg.include_patterns());

    let preprocessed_yaml = to_yaml(yaml_docs_filtered)?;
    output::print_to_stdout(&preprocessed_yaml, cfg.should_colorize())?;

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

fn parse_yaml(s: String) -> Vec<yaml_rust::Yaml> {
    let yaml_docs = yaml_rust::YamlLoader::load_from_str(s.as_str()).unwrap();
    yaml_docs
}

fn filter_yaml_docs(yaml_docs: Vec<yaml_rust::Yaml>, include_patterns: &[&str]) -> Vec<yaml_rust::Yaml> {
    yaml_docs.iter()
        .map(|doc| {
            yaml_parsing::filter_yaml(doc, "", &|key| {
                branch_selection::is_key_included(include_patterns, key)
            })
        })
        .collect()
}

fn to_yaml(yaml_docs: Vec<yaml_rust::Yaml>) -> Result<String, io::Error> {
    // Dump the YAML object
    let mut out_str = String::new();
    for doc in yaml_docs.iter() {
        let mut emitter = yaml_rust::YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap();
        out_str.push('\n');
    }
    Ok(out_str)
}
