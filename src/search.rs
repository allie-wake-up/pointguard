use crate::error::Result;
use crate::files;
use crate::opts::Show;
use crate::settings::Settings;
use crate::show;
use skim::prelude::*;
use std::io::{self, Cursor};
use walkdir::WalkDir;

pub fn search(buffer: &mut dyn io::Write, settings: Settings) -> Result<()> {
    let options = SkimOptionsBuilder::default().build().unwrap();
    let walker = WalkDir::new(&settings.dir).into_iter();
    let mut entries = Vec::new();
    let mut dir_str = settings
        .dir
        .to_str()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Point Guard Password Store Dir is not valid unicode",
            )
        })?
        .to_string();
    dir_str.push('/');
    for entry in walker.filter_entry(files::is_not_hidden) {
        let entry = match entry {
            Ok(entry) => entry,
            // TODO: should this return an error?
            Err(_e) => continue,
        };
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        entries.push(files::display_path(path, &dir_str[..])?);
    }
    entries.sort_unstable();

    // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
    // `SkimItem` was implemented for `AsRef<str>` by default
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(entries.join("\n")));

    // `run_with` would read and show items from the stream
    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

    if let Some(item) = selected_items.get(0) {
        show::show(
            buffer,
            Show::new(Some(item.output().to_string()), true),
            settings,
        )?;
    }

    Ok(())
}
