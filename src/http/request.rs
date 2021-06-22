use super::method::{HttpMethod, MethodError};
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{self, Utf8Error};

pub struct Request {
    pub path: String,
    pub query_string: Option<String>,
    pub method: HttpMethod,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let raw = str::from_utf8(buffer)?;

        let (method, path, protocol) = {
            let mut iterator = raw.trim().split_whitespace().take(3);
            (
                iterator
                    .next()
                    .ok_or(ParseError::InvalidRequest)?
                    .parse::<HttpMethod>()?,
                iterator
                    .next()
                    .ok_or(ParseError::InvalidRequest)?
                    .to_string(),
                iterator.next().ok_or(ParseError::InvalidRequest)?,
            )
        };

        let query_string = match path.find('?') {
            Some(index) => Some(path[index + 1..].to_string()),
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
