pub mod protocol;

#[cfg(test)]
mod tests {
    // use std::collections::HashMap;

    use crate::protocol;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Primitives {
        m: String,
    }

    #[test]
    fn primitives() {
        let primitives = Primitives {
            m: "hello".to_string(),
        };

        // Serialize
        let bytes = protocol::serializer::to_bytes(&primitives).unwrap();

        // Deserialize
        let deserialized_primitives =
            protocol::deserializer::from_bytes::<Primitives>(&bytes).unwrap();
        assert_eq!(primitives, deserialized_primitives);
    }

    //    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    //    struct CompundTypes {
    //        a: Vec<u8>,
    //        b: HashMap<String, u8>,
    //        c: Option<u8>,
    //        d: Option<String>,
    //        e: Primitives,
    //    }
    //
    //    #[test]
    //    fn compound_types() {
    //        let compound_types = CompundTypes {
    //            a: vec![1, 2, 3],
    //            b: [("a".to_string(), 1), ("b".to_string(), 2)]
    //                .iter()
    //                .cloned()
    //                .collect(),
    //            c: Some(1),
    //            d: None,
    //            e: Primitives {
    //                a: 1,
    //                b: 2,
    //                c: 3,
    //                d: 4,
    //                e: -1,
    //                f: -2,
    //                g: -3,
    //                h: -4,
    //                i: 1.0,
    //                j: 2.0,
    //                k: true,
    //                l: 'a',
    //                m: "hello".to_string(),
    //            },
    //        };
    //
    //        // Serialize
    //        let bytes = protocol::serializer::to_bytes(&compound_types).unwrap();
    //
    //        // Deserialize
    //        let deserialized_compound_types =
    //            protocol::deserializer::from_bytes::<CompundTypes>(&bytes).unwrap();
    //        assert_eq!(compound_types, deserialized_compound_types);
    //    }
    //
    //    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    //    struct Random {
    //        a: u8,
    //        b: u16,
    //        c: u32,
    //        d: u64,
    //        e: i8,
    //        f: i16,
    //        g: i32,
    //        h: i64,
    //        i: f32,
    //        j: f64,
    //        k: bool,
    //        l: char,
    //        m: String,
    //        n: Vec<u8>,
    //        o: HashMap<String, u8>,
    //        p: Option<u8>,
    //        q: Option<String>,
    //    }
    //
    //    #[test]
    //    fn random() {
    //        let random = Random {
    //            a: 1,
    //            b: 2,
    //            c: 3,
    //            d: 4,
    //            e: -1,
    //            f: -2,
    //            g: -3,
    //            h: -4,
    //            i: 1.0,
    //            j: 2.0,
    //            k: true,
    //            l: 'a',
    //            m: "hello".to_string(),
    //            n: vec![1, 2, 3],
    //            o: [("a".to_string(), 1), ("b".to_string(), 2)]
    //                .iter()
    //                .cloned()
    //                .collect(),
    //            p: Some(1),
    //            q: None,
    //        };
    //
    //        // Serialize
    //        let bytes = protocol::serializer::to_bytes(&random).unwrap();
    //
    //        // Deserialize
    //        let deserialized_random = protocol::deserializer::from_bytes::<Random>(&bytes).unwrap();
    //        assert_eq!(random, deserialized_random);
    //    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct Human {
        name: String,
        age: u8,
    }

    #[test]
    fn readme_example() {
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
