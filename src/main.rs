use protocol::error::Error;
use serde::{Deserialize, Serialize};
use std::fmt::Write;

mod protocol;

#[derive(Debug, Serialize, Deserialize)]
enum SomeEnum {
    A { a: u8, b: u16 },
    B(u8),
    C,
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    is_human: bool,
    languages: Vec<String>,
    hey: i32,
    hash_map: std::collections::HashMap<String, i32>,
    field1: SomeEnum,
    field2: Option<SomeEnum>,
    some_struct: SomeStruct,
}

#[derive(Debug, Serialize, Deserialize)]
struct SomeStruct {
    a: u8,
    b: u16,
}

fn main() -> Result<(), Error> {
    let person = Person {
        name: "Ayush".to_string(),
        age: 19,
        is_human: true,
        languages: vec!["English".to_string(), "Hindi".to_string()],
        hey: -123,
        hash_map: {
            let mut map = std::collections::HashMap::new();
            map.insert("one".to_string(), 1);
            map.insert("two".to_string(), 2);
            for i in 3..=100 {
                map.insert(i.to_string(), i);
            }
            map
        },
        field1: SomeEnum::A { a: 1, b: 2 },
        field2: None,
        some_struct: SomeStruct { a: 1, b: 2 },
    };
    println!("Data:\n{:?}\n", person);

    // Serialize
    let bytes = protocol::serializer::to_bytes(&person)?;
    println!("Serialized Length:\n{}\n", bytes.len());
    println!(
        "Serialized Bytes (hex):\n{}\n",
        bytes.iter().fold(String::new(), |mut output, b| {
            let _ = write!(output, "{b:02X}");
            output
        })
    );

    // Serialize with serde_json to Bytes
    let serde_bytes = serde_json::to_vec(&person).unwrap();
    println!("Serialized Length (serde_json):\n{}\n", serde_bytes.len());
    println!(
        "Serialized Bytes (hex) (serde_json):\n{}\n",
        serde_bytes.iter().fold(String::new(), |mut output, b| {
            let _ = write!(output, "{b:02X}");
            output
        })
    );

    // Deserialize
    let deserialized_person = protocol::deserializer::from_bytes::<Person>(&bytes)?;
    println!("Deserialized:\n{:?}\n", deserialized_person);

    Ok(())
}
