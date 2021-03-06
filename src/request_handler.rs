use super::http::{HttpMethod, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;
pub struct RequestHandler {
    public_path: String,
}

impl RequestHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(canonical_path) if canonical_path.starts_with(&self.public_path) => {
                fs::read_to_string(canonical_path).ok()
            }
            Ok(unauthorised_path) => {
                println!("Directory traversal detected: {:?}", unauthorised_path);
                None
            }
            Err(_) => None,
        }
    }
}

impl Handler for RequestHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            &HttpMethod::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/status" => Response::new(StatusCode::Ok, Some("Server is running!".to_string())),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
