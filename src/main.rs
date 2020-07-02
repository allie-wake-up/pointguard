mod gpg;
mod settings;
mod show;

use clap::Clap;
use settings::Settings;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Andrew Stephan<andrew@pointguard.dev>")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "default.conf")]
    config: String,
    /// Some input
    input: Option<String>,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(name = "test")]
    Test(Test),
    #[clap(name="show", aliases = &["ls", "show"])]
    Show(Show),
}

/// A subcommand for controlling testing
#[derive(Clap)]
struct Test {
    /// Print debug info
    #[clap(short)]
    debug: bool,
}

/// A subcommand for listing password files
#[derive(Clap)]
struct Show {
    /// Some input
    input: Option<String>,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    println!("Value for config: {}", opts.config);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match opts.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        _ => println!("Don't be crazy"),
    }

    let settings = Settings::new();
    let settings = settings.unwrap();

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd.unwrap_or(SubCommand::Show(Show { input: opts.input })) {
        SubCommand::Test(t) => {
            if t.debug {
                println!("Printing debug info...");
            } else {
                println!("Printing normally...");
            }
        }
        SubCommand::Show(opts) => {
            show::show(opts.input, settings).unwrap_or_else(|e| eprintln!("Error: {}", e));
        }
    }
}
