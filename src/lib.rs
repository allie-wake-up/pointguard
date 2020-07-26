mod clip;
mod error;
mod gpg;
mod opts;
mod settings;
mod show;

pub use error::PointGuardError;
pub use opts::{Opts, Show, SubCommand};
pub use settings::Settings;

pub fn run(buffer: &mut dyn std::io::Write, opts: Opts, settings: Settings) -> error::Result<()> {
    let input = opts.input;
    match opts
        .subcmd
        .unwrap_or_else(|| SubCommand::Show(Show::new(input)))
    {
        SubCommand::Clip => clip::clip(settings),
        SubCommand::Show(show_opts) => show::show(buffer, show_opts, settings),
    }
}
