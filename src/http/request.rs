use super::method::HttpMethod;
use std::convert::TryFrom;

pub struct Request {
    pub path: String,
    pub query_string: Option<String>,
    pub method: HttpMethod,
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

pub enum ParseError {
    InvalidRequest,  // General, non-specific
    InvalidEncoding, // If non-UTF-8
    InvalidProtocol, // Only supporting 1.1 (to begin with)
    InvalidMethod,   // If HTTP verb is malformed or unsupported
}
