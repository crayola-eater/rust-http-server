use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(&mut self) {
        println!("Server running on: {}!", self.address);

        let listener = TcpListener::bind(&self.address).expect("Failed to bind to TCP socket");
    }
}
