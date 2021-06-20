use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
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

        loop {
            match listener.accept() {
                Ok((mut stream, address)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(number_of_bytes) => {
                            println!(
                                "Received a request of size {} on {}: {}",
                                number_of_bytes,
                                address,
                                String::from_utf8_lossy(&buffer)
                            );
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => println!("Received request"),
                                Err(error) => println!("Failed to parse request. {}", error),
                            }
                        }
                        Err(error) => println!("Failed to read from connection. {}", error),
                    }
                }
                Err(error) => println!("Failed to establish a connection. {}", error),
            };
        }
    }
}
