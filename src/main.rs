#![allow(dead_code)]

use request_handler::RequestHandler;
use server::Server;

mod http;
mod request_handler;
mod server;

fn main() {
    let mut server = Server::new("127.0.0.1:8080".to_string());
    server.run(RequestHandler);
}
