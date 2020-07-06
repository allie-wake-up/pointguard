mod gpg;
mod opts;
mod settings;
mod show;

pub use opts::{Opts, Show, SubCommand};
pub use settings::Settings;

pub fn run(opts: Opts, settings: Settings) {
    let input = opts.input;
    match opts.subcmd.unwrap_or_else(|| SubCommand::Show(Show::new(input))) {
        SubCommand::Test(t) => {
            if t.debug {
                println!("Printing debug info...");
            } else {
                println!("Printing normally...");
            }
        }
        SubCommand::Show(sub_opts) => {
            show::show(sub_opts.input, settings).unwrap_or_else(|e| eprintln!("Error: {}", e));
        }
    }
}
