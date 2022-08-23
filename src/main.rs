extern crate yaml_rust;
use std::fs::File;
use std::io;
use std::io::Error;
use std::process;

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

fn is_key_included(include_patterns: &Vec<&str>, key: &str) -> bool {
    for p in include_patterns {
        // TODO: Allow * in all elements (compare element wise)
        if key.starts_with(p) || *p == "*" {
            return true;
        }
    }
    false
}

fn run(cfg: config::AppConfig) -> Result<(), io::Error> {
    let s = read_input(cfg.input.to_owned())?;
    for (i, line) in s.lines().enumerate() {
        println!("{:>3} |{}", i + 1, line);
    }

    let scanned_yaml =
        yaml_parsing::filter_documents(s, &|key| is_key_included(&cfg.include_patterns(), key))
            .unwrap();

    // let yaml_docs = parse_yaml(scanned_yaml);
    // let preprocessed_yaml = to_yaml(yaml_docs)?;

    output::print_to_stdout(scanned_yaml, cfg.should_colorize())?;

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

// fn parse_yaml(s: String) -> Vec<yaml_rust::Yaml> {
//     let yaml_docs = yaml_rust::YamlLoader::load_from_str(s.as_str()).unwrap();
//     yaml_docs
// }

// fn to_yaml(yaml_docs: Vec<yaml_rust::Yaml>) -> Result<String, io::Error> {
//     // Dump the YAML object
//     let mut out_str = String::new();
//     for doc in yaml_docs.iter() {
//         let mut emitter = yaml_rust::YamlEmitter::new(&mut out_str);
//         emitter.dump(doc).unwrap();
//         out_str.push('\n');
//     }
//     Ok(out_str)
// }
