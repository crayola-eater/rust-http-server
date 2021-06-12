fn main() {}

struct Server {
    address: String,
}

impl Server {
    fn new(address: String) -> Self {
        Self { address }
    }

    fn run(&mut self) {
        println!("Server is running! On: {}", self.address);
    }
}
