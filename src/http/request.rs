use super::method::HttpMethod;
use std::convert::TryFrom;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: HttpMethod,
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
