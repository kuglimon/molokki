use std::{
    fmt::{Display, Formatter},
    io, result,
};

pub type Result<T> = result::Result<T, RojektiError>;

#[derive(Debug)]
pub enum RojektiError {
    /// I/O related error
    Io(io::Error),

    /// Users runtime environment is some how busted. No PWD or HOME found for example.
    RuntimeError(String),

    /// Mostly unknown errors, which should be refactored to more concrete types
    Other(String),

    /// Configuration parsing error with path context (e.g. "$.windows[2].panes[0]")
    ParseError { path: String, message: String },
}

impl From<io::Error> for RojektiError {
    fn from(err: io::Error) -> RojektiError {
        RojektiError::Io(err)
    }
}

impl From<&str> for RojektiError {
    fn from(err: &str) -> RojektiError {
        RojektiError::Other(err.to_owned())
    }
}

impl Display for RojektiError {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), std::fmt::Error> {
        match self {
            RojektiError::Io(error) => write!(f, "IO error {}", error.to_string()),
            RojektiError::RuntimeError(error) => {
                write!(f, "RuntimeError error {}", error.to_string())
            }
            RojektiError::Other(error) => {
                write!(f, "Unknown error {}", error.to_string())
            }
            RojektiError::ParseError { path, message } => {
                write!(f, "Parse error at {}: {}", path, message)
            }
        }
    }
}
