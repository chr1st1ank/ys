use atty;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub(crate) struct AppConfig {
    /// Input file or "--" to read from stdin
    pub input: Option<String>,

    /// Enable monochrome output (no syntax highlighting)
    #[clap(long, short = 'm', action, default_value_t = false)]
    monochrome_output: bool,
}

impl AppConfig {
    pub fn from_args() -> AppConfig {
        AppConfig::parse()
    }
    pub fn should_colorize(&self) -> bool {
        if self.monochrome_output {
            return false;
        }
        atty::is(atty::Stream::Stdout)
    }
}
