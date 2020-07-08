use clap::Clap;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Andrew Stephan<andrew@pointguard.dev>")]
pub struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    // #[clap(short, long, default_value = "default.conf")]
    // config: String,
    /// Some input
    pub input: Option<String>,
    /// A level of verbosity, and can be used multiple times
    // #[clap(short, long, parse(from_occurrences))]
    // verbose: i32,
    #[clap(subcommand)]
    pub subcmd: Option<SubCommand>,
}

#[derive(Clap)]
pub enum SubCommand {
    #[clap(name = "test")]
    Test(Test),
    #[clap(name="show", aliases = &["ls", "show"])]
    Show(Show),
}

/// A subcommand for controlling testing
#[derive(Clap)]
pub struct Test {
    /// Print debug info
    #[clap(short)]
    pub debug: bool,
}

/// A subcommand for listing password files
#[derive(Clap)]
pub struct Show {
    /// Some input
    pub input: Option<String>,
}

impl Show {
    pub fn new(input: Option<String>) -> Self {
        Show { input }
    }
}
