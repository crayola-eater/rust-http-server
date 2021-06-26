#![allow(dead_code)]

use http::HttpMethod;
use http::Request;
use server::Server;

mod http;
mod request_handler;
mod server;

fn main() {
    let mut server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
