extern crate yaml_rust;
use clap::Parser;
use std::fs::File;
use std::io;
use std::io::Error;
use std::process;
use syntect::{easy, highlighting};

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct CommandLineArgs {
    /// Input file or "--" to read from stdin
    input: Option<String>,
}

fn main() {
    let args = CommandLineArgs::parse();
    let file_path = args.input;

    if let Err(e) = run(file_path) {
        error_exit(e);
    }
}

fn error_exit(e: Error) {
    eprintln!("Error opening file. {}", e);
    process::exit(1);
}

fn run(file_path: Option<String>) -> Result<(), io::Error> {
    let mut reader = open_input_stream(file_path)?;

    let mut s = String::new();
    (*reader).read_to_string(&mut s)?;
    let preprocessed_yaml = parse_yaml(s)?;

    let use_color = atty::is(atty::Stream::Stdout);
    print_output(preprocessed_yaml, use_color)?;

    Ok(())
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

fn open_input_stream(file_path: Option<String>) -> Result<Box<dyn io::BufRead>, io::Error> {
    if let Some(fp) = file_path {
        let file_handle = File::open(fp)?;
        Ok(Box::new(io::BufReader::new(file_handle)))
    } else {
        let file_handle = io::stdin().lock();
        Ok(Box::new(io::BufReader::new(file_handle)))
    }
}

fn print_output(text: String, use_color: bool) -> Result<(), io::Error> {
    if use_color {
        print_highlighted(&text);
    } else {
        for line in text.lines() {
            println!("{}", line);
        }
    }
    Ok(())
}

fn print_highlighted(text: &String) {
    let syntax_set = syntect::parsing::SyntaxSet::load_defaults_nonewlines();
    let theme_set = highlighting::ThemeSet::load_defaults();
    let theme = &theme_set.themes["Solarized (dark)"];
    let mut highlighter =
        easy::HighlightLines::new(syntax_set.find_syntax_by_extension("yml").unwrap(), theme);
    for line in text.lines() {
        let escaped = highlight_line(&mut highlighter, &syntax_set, line);
        println!("{}", escaped);
    }
}

fn highlight_line(
    highlighter: &mut easy::HighlightLines,
    syntax_set: &syntect::parsing::SyntaxSet,
    text: &str,
) -> String {
    let ranges: Vec<(highlighting::Style, &str)> =
        highlighter.highlight_line(text, syntax_set).unwrap();
    let escaped = syntect::util::as_24_bit_terminal_escaped(&ranges[..], true);
    escaped
}
