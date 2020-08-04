use crate::error::Result;
use crate::opts::Clip;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::io::{self, Read};

pub fn clip(opts: Clip) -> Result<()> {
    let mut clipboard = ClipboardContext::new()?;
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    clipboard.set_contents(buffer.trim_end().trim_end_matches('\n').to_owned())?;
    std::thread::sleep(std::time::Duration::from_secs(opts.clip_time));
    clipboard.clear()?;
    Ok(())
}
