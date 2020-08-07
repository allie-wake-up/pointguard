mod clip;
mod error;
mod files;
mod gpg;
mod opts;
mod search;
mod settings;
mod show;

pub use error::PointGuardError;
pub use opts::{Opts, Show, SubCommand};
pub use settings::Settings;

pub fn run(buffer: &mut dyn std::io::Write, opts: Opts, settings: Settings) -> error::Result<()> {
    let show_opts = opts.show;
    match opts.subcmd.unwrap_or_else(|| match &show_opts.input {
        Some(_) => SubCommand::Show(show_opts),
        None => SubCommand::Search,
    }) {
        SubCommand::Clip(clip_opts) => clip::clip(clip_opts),
        SubCommand::Show(show_opts) => show::show(buffer, show_opts, settings),
        SubCommand::Search => search::search(buffer, settings),
    }
}
