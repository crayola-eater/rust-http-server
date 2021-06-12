fn main() {
    let mut server = server::Server::new("127.0.0.1:8080".to_string());
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
    mod request {
        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: super::method::HttpMethod,
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
