fn main() {
    let get = HttpMethod::GET;
    let delete = HttpMethod::DELETE;
    let post = HttpMethod::POST;
    let put = HttpMethod::PUT;

    let mut server = server::Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server {
    struct Server {
        address: String,
    }

    impl Server {
        fn new(address: String) -> Self {
            Self { address }
        }

        fn run(&mut self) {
            println!("Server running on: {}!", self.address);
        }
    }
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: HttpMethod,
}

enum HttpMethod {
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
