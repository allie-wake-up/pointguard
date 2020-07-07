use crate::error::Result;
use crate::gpg;
use crate::opts::Show;
use crate::settings::Settings;
use ptree::output;
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
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Found a file with no name"))?
        .to_str()
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "File name is not valid unicode")
        })?
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

pub fn show(buffer: &mut dyn io::Write, opts: Show, settings: Settings) -> Result<()> {
    let path = match &opts.input {
        Some(name) => settings.dir.join(name),
        None => settings.dir,
    };
    let file = path.with_extension("gpg");
    println!("path: {:?}, file: {:?}", path, file);
    if !path.exists() && !file.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "{} is not in the point guard password store.",
                opts.input.unwrap_or_else(|| String::from("File or folder"))
            ),
        )
        .into());
    }
    if path.is_dir() {
        print_tree(&path, opts.input)
    } else {
        let pw = gpg::decrypt(&file)?;
        write!(buffer, "{}", pw)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn get_test_settings() -> Settings {
        Settings {
            dir: PathBuf::from("test-store-enc"),
            clip_time: 45,
            generated_length: 25,
            editor: String::from("vim"),
        }
    }

    #[test]
    fn print_password() {
        let mut result: Vec<u8> = vec![];
        show(
            &mut result,
            Show::new(Some(String::from("test"))),
            get_test_settings(),
        )
        .unwrap();
        assert_eq!(String::from_utf8(result).unwrap().trim(), "password");
    }

    #[test]
    fn print_website_password() {
        let mut result: Vec<u8> = vec![];
        show(
            &mut result,
            Show::new(Some(String::from("pointguard.dev"))),
            get_test_settings(),
        )
        .unwrap();
        assert_eq!(String::from_utf8(result).unwrap().trim(), "password");
    }
}
