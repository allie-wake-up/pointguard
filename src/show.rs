use crate::error::Result;
use crate::gpg;
use crate::opts::Show;
use crate::settings::Settings;
use anyhow::anyhow;
use ptree::output;
use std::{
    env,
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
};

mod pgtree;

fn clip(buffer: &mut dyn io::Write, pw: &str, clip_time: u64, opts: Show) -> Result<()> {
    let exe = env::current_exe()?;
    let mut child = Command::new(exe)
        .arg("clip")
        .arg(clip_time.to_string())
        .stdin(Stdio::piped())
        .spawn()?;
    let child_stdin = child.stdin.as_mut();
    let child_stdin =
        child_stdin.ok_or_else(|| anyhow!("Error launching child to copy to clipboard."))?;
    child_stdin.write_all(pw.as_bytes())?;
    writeln!(
        buffer,
        "Copied {} to clipboard. Will clear in {} seconds.",
        opts.input.unwrap(),
        clip_time
    )?;
    Ok(())
}

fn show_password(
    buffer: &mut dyn io::Write,
    file: &Path,
    clip_time: u64,
    opts: Show,
) -> Result<()> {
    let pw = gpg::decrypt(file)?;
    let pw = match &opts.line {
        Some(line) => pw
            .lines()
            .nth(line - 1)
            .ok_or_else(|| anyhow!("Error reading line {} of the password file", line)),
        None => Ok(&pw[..]),
    }?;
    if opts.clip {
        clip(buffer, pw, clip_time, opts)
    } else {
        match opts.line {
            Some(_) => writeln!(buffer, "{}", pw)?,
            None => write!(buffer, "{}", pw)?,
        }
        Ok(())
    }
}

pub fn show(buffer: &mut dyn io::Write, opts: Show, settings: Settings) -> Result<()> {
    let (path, file) = match &opts.input {
        Some(name) => (
            settings.dir.join(name),
            settings.dir.join(name.to_owned() + ".gpg"),
        ),
        None => (settings.dir.clone(), settings.dir),
    };
    if file.exists() && !file.is_dir() {
        show_password(buffer, &file, settings.clip_time, opts)
    } else if path.is_dir() {
        let root = pgtree::build_tree(&path, opts.input)?;
        output::write_tree(&root, buffer)?;
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "{} is not in the point guard password store.",
                opts.input.unwrap_or_else(|| String::from("File or folder"))
            ),
        )
        .into())
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
        assert_eq!(String::from_utf8(result).unwrap().trim(), "test\nline2");
    }

    #[test]
    fn print_password_line_2() {
        let mut result: Vec<u8> = vec![];
        let mut opts = Show::new(Some(String::from("test")));
        opts.line = Some(2);
        show(&mut result, opts, get_test_settings()).unwrap();
        assert_eq!(String::from_utf8(result).unwrap().trim(), "line2");
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
        assert_eq!(String::from_utf8(result).unwrap().trim(), "pointguard.dev");
    }

    #[test]
    fn print_password_with_same_name_as_dir() {
        let mut result: Vec<u8> = vec![];
        show(
            &mut result,
            Show::new(Some(String::from("same"))),
            get_test_settings(),
        )
        .unwrap();
        assert_eq!(String::from_utf8(result).unwrap().trim(), "same");
    }

    #[test]
    fn print_password_in_dir() {
        let mut result: Vec<u8> = vec![];
        show(
            &mut result,
            Show::new(Some(String::from("same/test"))),
            get_test_settings(),
        )
        .unwrap();
        assert_eq!(String::from_utf8(result).unwrap().trim(), "same/test");
    }

    #[test]
    fn print_root_tree() {
        let mut result: Vec<u8> = vec![];
        show(&mut result, Show::new(None), get_test_settings()).unwrap();
        let result_string = String::from_utf8(result).unwrap();
        assert!(result_string.contains("test"));
        assert!(result_string.contains("pointguard.dev"));
        assert!(result_string.contains("same"));
        assert!(result_string.contains("unique"));
        assert!(!result_string.contains("notinstore"));
    }

    #[test]
    fn print_tree_with_same_name_as_password() {
        let mut result: Vec<u8> = vec![];
        show(
            &mut result,
            Show::new(Some(String::from("same/"))),
            get_test_settings(),
        )
        .unwrap();
        let result_string = String::from_utf8(result).unwrap();
        assert!(result_string.contains("test"));
        assert!(result_string.contains("unique"));
        assert!(result_string.contains("same"));
        assert!(!result_string.contains("notinstore"));
    }
}
