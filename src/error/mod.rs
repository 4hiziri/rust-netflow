// TODO: Define here Error

use nom;
use nom::{Err, IResult};
use std::convert::From;
use std::error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error<'a> {
    ParseError(nom::Err<&'a [u8]>),
    InvalidLength,
    InvalidFieldValue, // TODO: contain wrong field name and val
    TemplateNotFound,
    UnexpectedIncomplete,
}

impl<'a> error::Error for Error<'a> {
    fn description(&self) -> &str {
        match *self {
            Error::ParseError(ref err) => err.description(),
            Error::InvalidLength => "Payload length is invalid",
            Error::InvalidFieldValue => "Field value is invalid",
            Error::TemplateNotFound => "Template is not found",
            Error::UnexpectedIncomplete => "Unexpected nom::IResult::Incomplete returned",
        }
    }
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::ParseError(err) => write!(f, "Parse error: {}", err),
            Error::InvalidLength => write!(f, "Payload length is invalid"),
            Error::InvalidFieldValue => write!(f, "Field value is invalid"),
            Error::TemplateNotFound => write!(f, "Template is not found"),
            Error::UnexpectedIncomplete => {
                write!(f, "Unexpected nom::IResult::Incomplete returned")
            }
        }
    }
}

impl<'a> From<nom::Err<&'a [u8]>> for Error<'a> {
    fn from(err: nom::Err<&'a [u8]>) -> Error<'a> {
        Error::ParseError(err)
    }
}

pub type ParseResult<'a, T> = Result<(&'a [u8], T), Error<'a>>;

// TODO: make new type
pub fn to_result<T>(res: IResult<&[u8], T>) -> Result<(&[u8], T), Error> {
    match res {
        Ok(ok) => Ok(ok),
        Err(Err::Incomplete(_)) => Err(Error::UnexpectedIncomplete),
        Err(e) => Err(Error::ParseError(e)),
    }
}
