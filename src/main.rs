use http::request::Request;
use server::Server;

mod server;

fn main() {
    let mut server = Server::new("127.0.0.1:8080".to_string());
    server.run();
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

    pub mod method {
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
