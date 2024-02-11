use serde::{Deserialize, Serialize};

mod serializer;
mod error;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    println!("Hello, world!");
}
