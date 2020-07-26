use crate::error::Result;
use crate::settings::Settings;
use std::io::{self, Read};
use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub fn clip(settings: Settings) -> Result<()> {
    let mut clipboard = ClipboardContext::new()?;
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    clipboard.set_contents(buffer.trim_end().trim_end_matches('\n').to_owned())?;
    std::thread::sleep(std::time::Duration::from_secs(settings.clip_time));
    clipboard.clear()?;
    Ok(())
}
