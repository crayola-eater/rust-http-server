use http::request::Request;
use server::Server;

fn main() {
    let mut server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server {
    pub struct Server {
        address: String,
    }

    impl Server {
        pub fn new(address: String) -> Self {
            Self { address }
        }

        pub fn run(&mut self) {
            println!("Server running on: {}!", self.address);
        }
    }
}

mod http {
    pub mod request {
        use super::method::HttpMethod;

        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: HttpMethod,
        }
    }

    mod method {
        pub enum HttpMethod {
            GET,
            DELETE,
            POST,
            PUT,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH,
        }
    }
}
