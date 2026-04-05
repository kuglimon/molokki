use std::{io, result};

pub type Result<T> = result::Result<T, RojektiError>;

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
