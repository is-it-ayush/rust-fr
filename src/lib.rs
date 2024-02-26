pub mod protocol;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::protocol;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Primitives {
        a: u8,
        b: u16,
        c: u32,
        d: u64,
        e: i8,
        f: i16,
        g: i32,
        h: i64,
        i: f32,
        j: f64,
        k: bool,
        l: char,
        m: String,
    }

    #[test]
    fn primitives() {
        let primitives = Primitives {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
            e: -1,
            f: -2,
            g: -3,
            h: -4,
            i: 1.0,
            j: 2.0,
            k: true,
            l: 'a',
            m: "hello".to_string(),
        };

        // Serialize
        let bytes = protocol::serializer::to_bytes(&primitives).unwrap();

        // Deserialize
        let deserialized_primitives =
            protocol::deserializer::from_bytes::<Primitives>(&bytes).unwrap();
        assert_eq!(primitives, deserialized_primitives);
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct CompundTypes {
        a: Vec<u8>,
        b: HashMap<String, u8>,
        c: Option<u8>,
        d: Option<String>,
        e: Primitives,
    }

    #[test]
    fn compound_types() {
        let compound_types = CompundTypes {
            a: vec![1, 2, 3],
            b: [("a".to_string(), 1), ("b".to_string(), 2)]
                .iter()
                .cloned()
                .collect(),
            c: Some(1),
            d: None,
            e: Primitives {
                a: 1,
                b: 2,
                c: 3,
                d: 4,
                e: -1,
                f: -2,
                g: -3,
                h: -4,
                i: 1.0,
                j: 2.0,
                k: true,
                l: 'a',
                m: "hello".to_string(),
            },
        };

        // Serialize
        let bytes = protocol::serializer::to_bytes(&compound_types).unwrap();

        // Deserialize
        let deserialized_compound_types =
            protocol::deserializer::from_bytes::<CompundTypes>(&bytes).unwrap();
        assert_eq!(compound_types, deserialized_compound_types);
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Random {
        a: u8,
        b: u16,
        c: u32,
        d: u64,
        e: i8,
        f: i16,
        g: i32,
        h: i64,
        i: f32,
        j: f64,
        k: bool,
        l: char,
        m: String,
        n: Vec<u8>,
        o: HashMap<String, u8>,
        p: Option<u8>,
        q: Option<String>,
    }

    #[test]
    fn random() {
        let random = Random {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
            e: -1,
            f: -2,
            g: -3,
            h: -4,
            i: 1.0,
            j: 2.0,
            k: true,
            l: 'a',
            m: "hello".to_string(),
            n: vec![1, 2, 3],
            o: [("a".to_string(), 1), ("b".to_string(), 2)]
                .iter()
                .cloned()
                .collect(),
            p: Some(1),
            q: None,
        };

        // Serialize
        let bytes = protocol::serializer::to_bytes(&random).unwrap();

        // Deserialize
        let deserialized_random = protocol::deserializer::from_bytes::<Random>(&bytes).unwrap();
        assert_eq!(random, deserialized_random);
    }

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

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct PlaygroundData {
        never: HashMap<String, Vec<u8>>,
        gonna: Vec<u8>,
        give: Option<i32>,
        you: bool,
        up: Option<Primitives>,
    }

    #[test]
    #[ignore = "playground test; use cargo test -- --nocapture --ignored"]
    fn length_test_large_data() {
        let data = PlaygroundData {
            never: (0..1000)
                .map(|i| (i.to_string(), vec![i as u8; 100]))
                .collect(),
            gonna: (0..1000).map(|i| i as u8).collect(),
            give: Some(1),
            you: true,
            up: Some(Primitives {
                a: 1,
                b: 2,
                c: 3,
                d: 4,
                e: -1,
                f: -2,
                g: -3,
                h: -4,
                i: 1.0,
                j: 2.0,
                k: true,
                l: 'a',
                m: "hello".to_string(),
            }),
        };

        let rust_fr_bytes = protocol::serializer::to_bytes(&data).unwrap();
        let serde_json_bytes = serde_json::to_vec(&data).unwrap();
        let rmp_serde_bytes = rmp_serde::to_vec(&data).unwrap();
        let mut cir_serde_bytes = Vec::new();
        ciborium::ser::into_writer(&data, &mut cir_serde_bytes).unwrap();

        println!("---- Large Data ----");
        println!("rust_fr:\t{} bytes", rust_fr_bytes.len());
        println!("serde_json:\t{} bytes", serde_json_bytes.len());
        println!("rmp_serde:\t{} bytes", rmp_serde_bytes.len());
        println!("ciborium:\t{} bytes", cir_serde_bytes.len());
    }

    #[test]
    #[ignore = "playground test; use cargo test -- --nocapture --ignored"]
    fn length_test_small_data() {
        let data = PlaygroundData {
            never: (0..10)
                .map(|i| (i.to_string(), vec![i as u8; 10]))
                .collect(),
            gonna: (0..10).map(|i| i as u8).collect(),
            give: Some(1),
            you: false,
            up: None,
        };

        let rust_fr_bytes = protocol::serializer::to_bytes(&data).unwrap();
        let serde_json_bytes = serde_json::to_vec(&data).unwrap();
        let rmp_serde_bytes = rmp_serde::to_vec(&data).unwrap();
        let mut cir_serde_bytes = Vec::new();
        ciborium::ser::into_writer(&data, &mut cir_serde_bytes).unwrap();

        println!("---- Small Data ----");
        println!("rust_fr:\t{} bytes", rust_fr_bytes.len());
        println!("serde_json:\t{} bytes", serde_json_bytes.len());
        println!("rmp_serde:\t{} bytes", rmp_serde_bytes.len());
        println!("ciborium:\t{} bytes", cir_serde_bytes.len());
    }

    #[test]
    #[ignore = "playground test; use cargo test -- --nocapture --ignored"]
    fn length_test_medium_data() {
        let data = PlaygroundData {
            never: (0..100)
                .map(|i| (i.to_string(), vec![i as u8; 100]))
                .collect(),
            gonna: (0..100).map(|i| i as u8).collect(),
            give: Some(1),
            you: true,
            up: Some(Primitives {
                a: 1,
                b: 2,
                c: 3,
                d: 4,
                e: -1,
                f: -2,
                g: -3,
                h: -4,
                i: 1.0,
                j: 2.0,
                k: true,
                l: 'a',
                m: "hello".to_string(),
            }),
        };

        let rust_fr_bytes = protocol::serializer::to_bytes(&data).unwrap();
        let serde_json_bytes = serde_json::to_vec(&data).unwrap();
        let rmp_serde_bytes = rmp_serde::to_vec(&data).unwrap();
        let mut cir_serde_bytes = Vec::new();
        ciborium::ser::into_writer(&data, &mut cir_serde_bytes).unwrap();

        println!("---- Medium Data ----");
        println!("rust_fr:\t{} bytes", rust_fr_bytes.len());
        println!("serde_json:\t{} bytes", serde_json_bytes.len());
        println!("rmp_serde:\t{} bytes", rmp_serde_bytes.len());
        println!("ciborium:\t{} bytes", cir_serde_bytes.len());
    }
}
