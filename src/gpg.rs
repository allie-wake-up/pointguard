use std::path::Path;
use std::process::{Command, Output};
use std::io::Result;

const DEFAULT_ARGS: &[&str] = &[
    "--quiet",
    "--yes",
    "--compress-algo=none",
    "--no-encrypt-to",
];

pub fn decrypt(path: &Path) -> Result<Output> {
    Command::new("gpg")
        .args(DEFAULT_ARGS)
        .arg("-d")
        .arg(path)
        .output()
}

