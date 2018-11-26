use nom::{Err, IResult};
use std::error::Error;

#[derive(Debug, Fail)]
pub enum NetFlowError {
    // delegating description
    #[fail(display = "{}", desc)]
    ParseError { desc: String },
    #[fail(display = "Invalid payload length")]
    InvalidLength,
    #[fail(display = "Invalid field value")]
    InvalidFieldValue, // TODO: contain wrong field name and val
    #[fail(display = "Template not found")]
    TemplateNotFound,
    #[fail(display = "Invalid netflow packet")]
    UnexpectedIncomplete,
}

pub type ParseResult<'a, T> = Result<(&'a [u8], T), NetFlowError>;

pub fn to_result<T>(res: IResult<&[u8], T>) -> Result<(&[u8], T), NetFlowError> {
    match res {
        Ok(ok) => Ok(ok),
        Err(Err::Incomplete(_)) => Err(NetFlowError::UnexpectedIncomplete),
        Err(e) => Err(NetFlowError::ParseError {
            desc: e.description().to_string(),
        }),
    }
}
