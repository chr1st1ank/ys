use clap::Parser;
use std::fs::File;
use std::io;
use std::io::{BufRead, Error};
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
    write_out(reader, true)?;
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

fn write_out(stream: Box<dyn io::BufRead>, formatted: bool) -> Result<(), io::Error> {
    if formatted {
        let syntax_set = syntect::parsing::SyntaxSet::load_defaults_nonewlines();
        let theme_set = highlighting::ThemeSet::load_defaults();
        let mut highlighter = easy::HighlightLines::new(
            syntax_set.find_syntax_by_extension("yml").unwrap(),
            &theme_set.themes["Solarized (dark)"],
        );

        for line in stream.lines() {
            let str_line = line?;
            let escaped = highlight_for_shell(&mut highlighter, &syntax_set, str_line);
            println!("{}", escaped);
        }
    } else {
        for line in stream.lines() {
            println!("{}", line?);
        }
    }
    Ok(())
}

fn highlight_for_shell(
    highlighter: &mut easy::HighlightLines,
    syntax_set: &syntect::parsing::SyntaxSet,
    str_line: String,
) -> String {
    let ranges: Vec<(highlighting::Style, &str)> = highlighter
        .highlight_line(str_line.as_str(), syntax_set)
        .unwrap();
    let escaped = syntect::util::as_24_bit_terminal_escaped(&ranges[..], true);
    escaped
}
