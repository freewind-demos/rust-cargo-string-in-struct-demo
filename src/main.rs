struct Hello {
    name: String,
}

impl Hello {
    pub fn hello(&self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let hello = Hello { name: "Rust".to_string() };
    hello.hello();
}
