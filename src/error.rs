use thiserror::Error;

#[derive(Error, Debug)]
pub enum PointGuardError {
    #[error("GPG Error: {1}")]
    GpgError(i32, String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, PointGuardError>;
