extern crate yaml_rust;
use std::fs::File;
use std::io::Error;
use std::process;
use std::{clone, io};

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
    for (i, line) in s.lines().enumerate() {
        println!("{:>3} |{}", i + 1, line);
    }

    // let sections = yaml_parsing::find_sections(s.clone()).unwrap();
    // for s in sections {
    //     println!("{:?}", s);
    // }

    // let scanned_yaml = yaml_parsing::filter_documents(s, &|key| {
    //     yaml_parsing::is_key_included(&cfg.include_patterns(), key)
    // })
    // .unwrap();

    let yaml_docs = parse_yaml(s);
    let yaml_docs_filtered = yaml_docs
        .iter()
        .map(|doc| {
            filter_yaml(doc, "", &|key| {
                let b = yaml_parsing::is_key_included(&cfg.include_patterns(), key);
                println!("{} {}", key, b);
                b
            })
        })
        .collect();
    let preprocessed_yaml = to_yaml(yaml_docs_filtered)?;

    output::print_to_stdout(preprocessed_yaml, cfg.should_colorize())?;

    Ok(())
}

fn filter_yaml(
    yaml_element: &yaml_rust::Yaml,
    path: &str,
    is_key_whitelisted: &dyn Fn(&str) -> bool,
) -> yaml_rust::Yaml {
    use yaml_rust::Yaml;

    match yaml_element {
        Yaml::Real(_) | Yaml::Integer(_) | Yaml::String(_) | Yaml::Boolean(_) | Yaml::Null => {
            yaml_element.clone()
        }
        Yaml::Array(arr) => filter_array(arr, path, &is_key_whitelisted),
        Yaml::Hash(arr) => filter_map(arr, path, &is_key_whitelisted),
        _ => yaml_element.clone(),
    }
}

fn filter_array(
    yaml_array: &Vec<yaml_rust::Yaml>,
    path: &str,
    is_key_whitelisted: &dyn Fn(&str) -> bool,
) -> yaml_rust::Yaml {
    use yaml_rust::Yaml;
    println!("filter_array({:?})", yaml_array);
    let vec2 = yaml_array
        .iter()
        .enumerate()
        .map(|(i, y)| (i, y, concat_path(path, &i.to_string())))
        .filter(|(i, y, p)| is_key_whitelisted(p))
        .map(|(i, y, p)| filter_yaml(y, &p, &is_key_whitelisted))
        .collect();
    Yaml::Array(vec2)
}

fn filter_map(
    yaml_map: &yaml_rust::yaml::Hash,
    path: &str,
    is_key_whitelisted: &dyn Fn(&str) -> bool,
) -> yaml_rust::Yaml {
    use yaml_rust::Yaml;
    let map2 = yaml_map
        .iter()
        .map(|(k, v)| (k, v, concat_path(path, &yaml_to_string(k))))
        .filter(|(_, _, p)| is_key_whitelisted(p))
        .map(|(k, v, p)| (k.to_owned(), filter_yaml(v, &p, &is_key_whitelisted)))
        .collect();
    Yaml::Hash(map2)
}

fn concat_path(path: &str, k: &str) -> String {
    if path == "" {
        k.to_owned()
    } else {
        (path.to_owned() + "." + k).to_owned()
    }
}

fn yaml_to_string(yaml_element: &yaml_rust::Yaml) -> String {
    use yaml_rust::Yaml;

    match yaml_element {
        Yaml::String(s) => s.clone(),
        Yaml::Boolean(b) => b.to_string(),
        Yaml::Real(n) => n.clone(),
        Yaml::Integer(n) => n.to_string(),
        _ => "".to_owned(),
    }
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
