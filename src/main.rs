use second::error::Error;
use serde::{Deserialize, Serialize};

mod second;

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
        field1: SomeEnum::A { a: 1, b: 2 },
        field2: None,
        some_struct: SomeStruct { a: 1, b: 2 },
    };
    println!("{:?}\n", person);

    let bytes = second::serializer::to_bytes(&person)?;

    println!("{}", bytes.len());
    println!(
        "{}\n",
        bytes
            .iter()
            .map(|b| format!("{:02x} ", b))
            .collect::<String>()
    );

    let deserialized_person = second::deserializer::from_bytes::<Person>(&bytes)?;
    println!("Deserialized: \n{:?}\n", deserialized_person);

    Ok(())
}
