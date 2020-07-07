mod error;
mod gpg;
mod opts;
mod settings;
mod show;

pub use error::PointGuardError;
pub use opts::{Opts, Show, SubCommand};
pub use settings::Settings;

pub fn run(opts: Opts, settings: Settings) -> error::Result<()> {
    let input = opts.input;
    match opts
        .subcmd
        .unwrap_or_else(|| SubCommand::Show(Show::new(input)))
    {
        SubCommand::Test(t) => {
            if t.debug {
                println!("Printing debug info...");
                Ok(())
            } else {
                println!("Printing normally...");
                Ok(())
            }
        }
        SubCommand::Show(show_opts) => show::show(show_opts, settings),
    }
}
