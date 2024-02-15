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
    age: u8,
    money: f64,
    is_human: bool,
    has_pet: Option<bool>,
    need_to_eat: Option<bool>,
    last_grade: SomeEnum,
    is_student: Option<Vec<bool>>,
    languages: Vec<String>,
}

fn main() {
    let person = Person {
        name: "Ayush".to_string(),
        age: 19,
        money: 0.69,
        is_human: true,
        has_pet: Some(false),
        need_to_eat: Some(true),
        last_grade: SomeEnum::C,
        is_student: Some(vec![true, false, true]),
        languages: ["Rust", "TypeScript", "C"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
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
