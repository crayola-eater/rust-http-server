use super::http::{Request, Response, StatusCode};
use super::server::Handler;
pub struct RequestHandler {}

impl Handler for RequestHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        unimplemented!()
    }
}
