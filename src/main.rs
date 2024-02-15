use serde::{Deserialize, Serialize};

use crate::error::CustomError;

mod deserializer;
mod error;
mod serializer;

#[derive(Debug, Serialize, Deserialize)]
enum SomeEnum {
    A,
    B,
    C,
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
}

fn main() {
    let person = Person {

        name: "Ayush".to_string(),
    };

    println!("Original Data: {:?}\n", person);

    let bytes = serializer::to_bytes(&person).unwrap();
    println!(
        "Serialized Data: {:?}\n",
        bytes
            .iter()
            .map(|&i| format!("{:02x}", i))
            .collect::<Vec<String>>()
            .join(" ")
    );
    println!("Serialized Data Length: {}\n", bytes.len());

    let deserialized_person = deserializer::from_bytes::<Person>(&bytes)
        .map_err(|e| CustomError::DeserializationError(e.to_string()));
    println!("Deserialized Data: {:?}\n", deserialized_person);

    // let binary: String = bytes
    //     .iter()
    //     .map(|&i| format!("{:08b}", i))
    //     .collect::<Vec<String>>()
    //     .join(" ");

    // println!("Binary Stream: {}", binary);

    // let hex: String = bytes
    //     .iter()
    //     .map(|&i| format!("{:02x}", i))
    //     .collect::<Vec<String>>()
    //     .join(" ");

    // println!("Hex Stream: {}", hex);
}
