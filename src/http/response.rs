pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }
}

pub enum StatusCode {}
