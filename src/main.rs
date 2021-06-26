#![allow(dead_code)]

use request_handler::RequestHandler;
use server::Server;
use std::env;

mod http;
mod request_handler;
mod server;

fn main() {
    let public_path =
        env::var("PUBLIC_PATH").expect("No PUBLIC_PATH environment variable detected");
    let mut server = Server::new("127.0.0.1:8080".to_string());
    server.run(RequestHandler);
}
