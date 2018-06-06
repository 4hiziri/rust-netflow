// TODO: Define here Error

use nom;
use std::convert::From;
use std::error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    ParseError(nom::Err),
    InvalidLength,
    InvalidFieldValue, // TODO: contain wrong field name and val
    TemplateNotFound,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ParseError(ref err) => err.description(),
            Error::InvalidLength => "Payload length is invalid",
            Error::InvalidFieldValue => "Field value is invalid",
            Error::TemplateNotFound => "Template is not found",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::ParseError(err) => write!(f, "Parse error: {}", err),
            Error::InvalidLength => write!(f, "Payload length is invalid"),
            Error::InvalidFieldValue => write!(f, "Field value is invalid"),
            Error::TemplateNotFound => write!(f, "Template is not found"),
        }
    }
}

impl From<nom::Err> for Error {
    fn from(err: nom::Err) -> Error {
        Error::ParseError(err)
    }
}

pub type ParseResult<'a, T> = Result<(&'a [u8], T), Error>;
