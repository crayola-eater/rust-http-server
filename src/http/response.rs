use super::StatusCode;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let body = match &self.body {
            Some(body) => body,
            None => "",
        };

        write!(
            f,
            "HTTP/1.1 {code} {reason}\r\n\r\n{body}",
            code = self.status_code,
            reason = self.status_code.reason_phrase(),
            body = body,
        )
    }
}
