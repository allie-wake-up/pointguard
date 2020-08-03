use clap::Clap;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Allie Stephan<allie@pointguard.dev>")]
pub struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    // #[clap(short, long, default_value = "default.conf")]
    // config: String,
    /// A level of verbosity, and can be used multiple times
    // #[clap(short, long, parse(from_occurrences))]
    // verbose: i32,
    #[clap(subcommand)]
    pub subcmd: Option<SubCommand>,
    #[clap(flatten)]
    pub show: Show,
}

#[derive(Clap)]
pub enum SubCommand {
    #[clap(name = "clip")]
    Clip(Clip),
    #[clap(name="show", aliases = &["ls", "show"])]
    Show(Show),
}

/// A subcommand for copying to the clipboard
#[derive(Clap)]
pub struct Clip {
    /// The number of seconds to copy the input
    /// for before clearing the clipboard
    pub clip_time: u64,
}

/// A subcommand for listing password files
#[derive(Clap)]
pub struct Show {
    /// A password file or directory to display
    pub input: Option<String>,
    /// Copy to clipboard instead of printing
    #[clap(name = "clip", long, short)]
    pub clip: bool,
    /// Line number to print or copy (starts with 1).
    #[clap(name = "line", long, short)]
    pub line: Option<usize>,
}

impl Show {
    pub fn new(input: Option<String>) -> Self {
        Show {
            input,
            clip: false,
            line: None,
        }
    }
}
