use std::io;
use std::io::Write;
use syntect;
use syntect::easy;
use syntect::highlighting;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// Print the given text to stdout
pub(crate) fn print_to_stdout(text: &str, use_color: bool) -> Result<(), io::Error> {
    if use_color {
        print_highlighted(&text)?;
    } else {
        for line in text.lines() {
            println!("{}", line);
        }
    }
    Ok(())
}

fn print_highlighted(text: &str) -> io::Result<()> {
    let syntax_set = syntect::parsing::SyntaxSet::load_defaults_nonewlines();
    let theme_set = highlighting::ThemeSet::load_defaults();
    let theme = &theme_set.themes["Solarized (dark)"];
    let mut highlighter =
        easy::HighlightLines::new(syntax_set.find_syntax_by_extension("yml").unwrap(), theme);
    for (i, line) in text.lines().enumerate() {
        let escaped = highlight_line(&mut highlighter, &syntax_set, line);
        write_line_number(i + 1)?;
        println!("{}", escaped);
    }
    Ok(())
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

fn write_line_number(i: usize) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::White))
            .set_bg(Some(Color::Black)),
    )?;
    write!(&mut stdout, "{:>5} ", i)
}
