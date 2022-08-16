use std::io;
use syntect;
use syntect::easy;
use syntect::highlighting;

/// Print the given text to stdout
pub(crate) fn print_to_stdout(text: String, use_color: bool) -> Result<(), io::Error> {
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
