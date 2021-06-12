fn main() {
    let mut server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

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

struct Request {
    path: String,
    query_string: String,
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
