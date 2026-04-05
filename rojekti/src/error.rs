use std::{io, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(Box<RojektiError>);

impl Error {
    pub fn new(kind: RojektiError) -> Error {
        Error(Box::new(kind))
    }
}

#[derive(Debug)]
pub enum RojektiError {
    /// I/O related error
    Io(io::Error),

    /// Templating related errors
    TemplateError(),

    /// Users runtime environment is some how busted. No PWD or HOME found for example.
    RuntimeError(String),

    /// Mostly unknown errors, which should be refactored to more concrete types
    Other(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::new(RojektiError::Io(err))
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Error {
        Error::new(RojektiError::Other(err.to_owned()))
    }
}
