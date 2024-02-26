pub mod protocol;

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::protocol;

    #[test]
    fn serialize_and_deserialize_complex_data() {
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
        enum SomeEnum {
            A { a: u8, b: u16 },
            B(u8),
            C,
        }

        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
        struct Person {
            name: String,
            age: u8,
            is_human: bool,
            some_enum: SomeEnum,
            som_other_enum: SomeEnum,
            some_struct: SomeStruct,
        }

        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
        struct SomeStruct {
            a: u8,
            b: u16,
        }

        let person = Person {
            name: "Ayush".to_string(),
            age: 19,
            is_human: false,
            some_enum: SomeEnum::A { a: 142, b: 5156 },
            som_other_enum: SomeEnum::B(4),
            some_struct: SomeStruct { a: 32, b: 51 },
        };

        // Serialize
        let bytes = protocol::serializer::to_bytes(&person).unwrap();

        // Deserialize
        let deserialized_person = protocol::deserializer::from_bytes::<Person>(&bytes).unwrap();
        assert_eq!(person, deserialized_person);
    }

    #[test]
    fn readme_example() {
        // define some data
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
        struct Human {
            name: String,
            age: u8,
        }

        let human = Human {
            name: "Ayush".to_string(),
            age: 19,
        };

        // serialize the data to bytes (Vec<u8>)
        let human_bytes = protocol::serializer::to_bytes(&human).unwrap();

        // deserialize the data from serialized bytes.
        let deserialized_human = protocol::deserializer::from_bytes::<Human>(&human_bytes).unwrap();

        assert_eq!(human, deserialized_human);
    }
}
