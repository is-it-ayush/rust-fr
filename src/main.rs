use serde::{Deserialize, Serialize};

mod error;
mod serializer;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    let bytes = serializer::to_bytes(&person).unwrap();
    println!("{:?}", bytes);
}
