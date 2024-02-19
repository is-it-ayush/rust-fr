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
    some_enum: SomeEnum,
    som_other_enum: SomeEnum,
    some_struct: SomeStruct,
    hash_map: std::collections::HashMap<String, i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SomeStruct {
    a: u8,
    b: u16,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}

fn run() -> Result<(), Error> {
    let person = Person {
        name: "Ayush".to_string(),
        age: 19,
        is_human: false,
        some_enum: SomeEnum::A { a: 142, b: 5156 },
        som_other_enum: SomeEnum::B(4),
        some_struct: SomeStruct { a: 32, b: 51 },
        hash_map: {
            let mut map = std::collections::HashMap::new();
            for i in 1..=100 {
                map.insert(i.to_string(), i * 10000);
            }
            map
        },
    };
    println!("Data:\n{:?}\n", person);

    // Serialize
    let bytes = protocol::serializer::to_bytes(&person)?;
    println!("Serialized Length (mine): {}", bytes.len());
    //println!(
    //    "Serialized Bytes:\n{}\n",
    //    bytes.iter().fold(String::new(), |mut s, b| {
    //          write!(&mut s, "{:02x}", b).unwrap();
    //          s
    //    })
    //);

    // Serialize with serde_json to Bytes
    let serde_bytes = serde_json::to_vec(&person).unwrap();
    println!("Serialized Length (serde_json): {}", serde_bytes.len());

    // Serialize with ciborium to Bytes
    let mut ciborium_bytes = Vec::new();
    ciborium::into_writer(&person, &mut ciborium_bytes).unwrap();
    println!("Serialized Length (ciborium): {}", ciborium_bytes.len());
    // println!(
    //     "{}\n",
    //     ciborium_bytes.iter().fold(String::new(), |mut s, b| {
    //         write!(&mut s, "{:02x}", b).unwrap();
    //         s
    //     })
    // );

    // Serialize with rmp to Bytes
    let mut rmp_bytes = rmp_serde::to_vec(&person).unwrap();
    println!("Serialized Length (rmp): {}", rmp_bytes.len());
    //println!(
    //    "{}\n",
    //    rmp_bytes.iter().fold(String::new(), |mut s, b| {
    //        write!(&mut s, "{:02x}", b).unwrap();
    //        s
    //    })
    //);

    // Deserialize
    let deserialized_person = protocol::deserializer::from_bytes::<Person>(&bytes)?;
    println!("\nDeserialized:\n{:?}\n", deserialized_person);

    Ok(())
}
