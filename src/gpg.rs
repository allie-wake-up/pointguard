use crate::error::PointGuardError;
use std::path::Path;
use std::process::Command;

const DEFAULT_ARGS: &[&str] = &[
    "--quiet",
    "--yes",
    "--compress-algo=none",
    "--no-encrypt-to",
];

pub fn decrypt(path: &Path) -> Result<String, PointGuardError> {
    let output = Command::new("gpg")
        .args(DEFAULT_ARGS)
        .arg("-d")
        .arg(path)
        .output()?;

    if output.status.success() {
        let result = String::from_utf8(output.stdout)?;
        Ok(result)
    } else {
        let error = String::from_utf8(output.stderr)?;
        Err(PointGuardError::GpgError(output.status.code().unwrap_or(1), error))
    }
}
