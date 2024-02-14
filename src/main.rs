use serde::{Deserialize, Serialize};

mod error;
mod serializer;
mod deserializer;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    is_human: bool,
    languages: Vec<String>,
}

fn main() {
    let person = Person {
        name: "Ayush".to_string(),
        age: 19,
        is_human: true,
        languages: ["Rust", "TypeScript", "C"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    };

    println!("Data: {:?}", person);

    let bytes = serializer::to_bytes(&person).unwrap();
    println!("Unsiged Bytes: {:?}", bytes);

    let binary: String = bytes
        .iter()
        .map(|&i| format!("{:08b}", i))
        .collect::<Vec<String>>()
        .join(" ");

    println!("Binary Stream: {}", binary);

    let hex: String = bytes
        .iter()
        .map(|&i| format!("{:02x}", i))
        .collect::<Vec<String>>()
        .join(" ");

    println!("Hex Stream: {}", hex);
}
