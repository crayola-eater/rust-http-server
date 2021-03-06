use super::method::{HttpMethod, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{self, Utf8Error};

#[derive(Debug)]
pub struct Request<'buffer> {
    path: &'buffer str,
    query_string: Option<QueryString<'buffer>>,
    method: HttpMethod,
}

impl<'buffer> Request<'buffer> {
    pub fn path(&self) -> &'buffer str {
        &self.path
    }

    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buffer> TryFrom<&'buffer [u8]> for Request<'buffer> {
    type Error = ParseError;

    fn try_from(buffer: &'buffer [u8]) -> Result<Self, Self::Error> {
        let raw = str::from_utf8(buffer)?;

        let (method, path, _protocol) = {
            let mut iterator = raw.trim().split_whitespace().take(3);
            (
                iterator
                    .next()
                    .ok_or(ParseError::InvalidRequest)?
                    .parse::<HttpMethod>()?,
                iterator.next().ok_or(ParseError::InvalidRequest)?,
                iterator.next().ok_or(ParseError::InvalidRequest)?,
            )
        };

        let query_string = match path.find('?') {
            Some(index) => Some(QueryString::from(&path[index + 1..])),
            None => None,
        };

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

pub enum ParseError {
    InvalidRequest,  // General, non-specific
    InvalidEncoding, // If non-UTF-8
    InvalidProtocol, // Only supporting 1.1 (to begin with)
    InvalidMethod,   // If HTTP verb is malformed or unsupported
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid request",
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidProtocol => "Invalid protocol",
            Self::InvalidMethod => "Invalid method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}
