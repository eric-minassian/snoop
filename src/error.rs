use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Json(serde_json::Error),
    PackageNotFound(String),
    CommandNotFound(String),
    MissingPackageName,
    CommandExecutionFailed(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {}", err),
            Self::Json(err) => write!(f, "JSON parsing error: {}", err),
            Self::PackageNotFound(name) => write!(f, "Package not found: {}", name),
            Self::CommandNotFound(name) => write!(f, "Command not found: {}", name),
            Self::MissingPackageName => write!(f, "Package name is required for workspaces"),
            Self::CommandExecutionFailed(err) => write!(f, "Command execution failed: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) | Self::CommandExecutionFailed(err) => Some(err),
            Self::Json(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(_: std::time::SystemTimeError) -> Self {
        Self::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid system time",
        ))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
