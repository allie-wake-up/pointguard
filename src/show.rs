use crate::settings::Settings;
use crate::gpg;
use ptree::output;
use std::io::{Error, ErrorKind, Result, Write};
use std::io;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

mod pgtree;
use pgtree::TreeBuilder;

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn display_path(path: &Path) -> Result<String> {
    Ok(path
        .file_stem()
        .ok_or_else(|| Error::new(
            ErrorKind::InvalidData,
            "Found a file with no name",
        ))?
        .to_str()
        .ok_or_else(|| Error::new(
            ErrorKind::InvalidData,
            "File name is not valid unicode",
        ))?
        .to_string())
}

fn print_tree(path: &Path, input: Option<String>) -> Result<()> {
    let mut builder =
        TreeBuilder::new(input.unwrap_or_else(|| String::from("Point Guard Password Store")));
    let walker = WalkDir::new(&path).into_iter();
    let mut depth = 1;
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = match entry {
            Ok(entry) => entry,
            // TODO: should this return an error?
            Err(_e) => continue,
        };
        if entry.depth() == 0 {
            continue;
        }
        let path = entry.path();
        if path.is_dir() {
            builder.begin_child(display_path(path)?);
            depth += 1;
        } else if entry.depth() == depth {
            builder.add_empty_child(display_path(path)?);
        } else {
            builder.end_child();
            builder.add_empty_child(display_path(path)?);
            depth -= 1;
        }
    }
    let mut root = builder.build();
    root.sort();
    output::print_tree(&root)?;
    Ok(())
}

fn show_password(file: &Path) -> Result<()> {
    let output = gpg::decrypt(&file)?;
    if output.status.success() {
        io::stdout().write_all(&output.stdout)?;
    } else {
        io::stderr().write_all(&output.stderr)?;
    }
    Ok(())
}

pub fn show(input: Option<String>, settings: Settings) -> Result<()> {
    let path = match &input {
        Some(name) => settings.dir.join(name),
        None => settings.dir,
    };
    let file = path.with_extension("gpg");
    if !path.exists() && !file.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!(
                "{} is not in the point guard password store.",
                input.unwrap_or_else(|| String::from("File or folder"))
            ),
        ));
    }
    if path.is_dir() {
        print_tree(&path, input)
    } else {
        show_password(&file)
    }
}
