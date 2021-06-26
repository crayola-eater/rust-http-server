use super::http::{HttpMethod, Request, Response, StatusCode};
use super::server::Handler;
pub struct RequestHandler {
    public_path: String,
}

impl Handler for RequestHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            &HttpMethod::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>Welcome</h1>".to_string())),
                "/status" => Response::new(StatusCode::Ok, Some("Server is running!".to_string())),
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
