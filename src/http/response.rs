use super::StatusCode;
use std::io::{Result as IoResult, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }

    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
        let body = match &self.body {
            Some(body) => body,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {code} {reason}\r\n\r\n{body}",
            code = self.status_code,
            reason = self.status_code.reason_phrase(),
            body = body,
        )
    }
}
