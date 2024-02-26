### rust-fr

'rust-fr' (aka `rust for real`) is a simple, non-self-describing data-interchange format.

### installation

You can use either of these methods.

- Add via `cargo add rust-fr`
- Add via `Cargo.toml`
```.toml
[dependencies]
rust-fr = "1"
```

### usage.

```rs
use serde::{Serialize, Deserialize};
use rust_fr::{serializer, deserializer};

// define some data
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Human {
    name: String,
    age: u8
};
let human = Human {
    name: "Ayush".to_string(),
    age: 19
};

// serialize the data to bytes (Vec<u8>)
let human_bytes = serializer::to_bytes(&human).unwrap();

// deserialize the data from serialized bytes.
let deserialized_human = deserializer::from_bytes::<Human>(&human_bytes).unwrap();

assert_eq!(human, deserialized_human);
```

### benchmark.

- Run `cargo test -- --nocapture --ignored` to run the benchmark tests.
```sh
running 3 tests
---- Small Data ----
rust_fr:        218 bytes
serde_json:     332 bytes
rmp_serde:      146 bytes
ciborium:       170 bytes
test tests::length_test_small_data ... ok
---- Medium Data ----
rust_fr:        14264 bytes
serde_json:     30125 bytes
rmp_serde:      10731 bytes
ciborium:       18347 bytes
test tests::length_test_medium_data ... ok
---- Large Data ----
rust_fr:        139214 bytes
serde_json:     367595 bytes
rmp_serde:      157219 bytes
ciborium:       198277 bytes
test tests::length_test_large_data ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.01s
```

### why?

The goal was to learn/understand. I wrote this so I can learn how serde internally works
and how to encode data into bytes that can ultimately be transferred over the wire
or elsewhere.

### format specification.

- The format is not self-describing.
- Primitive types are serialized as is.
    - bool: 0 -> false, 1 -> true (1 byte)
    - i8, i16, i32, i64: as is.
    - u8, u16, u32, u64: as is.
    - f32, f64: as is.
    - char: as u32 (4 bytes)
- Delimiters are used to separate different types of data.
- String, Byte and Map Delimiters are 1 byte long while all other delimiters are 3 bits long.
- Delimiters:
    - String = 134; 0b10000110
    - Byte = 135; 0b10000111
    - Unit = 2; 0b010
    - Seq = 3; 0b011
    - SeqValue = 4; 0b100
    - Map = 139; 0b10001011
    - MapKey = 6; 0b110
    - MapValue = 7; 0b111
- String, Bytes, Unit, Option are serialized as:
    - str: bytes + STRING_DELIMITER
    - bytes: bytes + BYTE_DELIMITER
    - unit: UNIT (null)
    - option: None -> unit(), Some -> self
- Structs are serialized as:
    - unit_struct: unit()
    - newtype_struct: self
    - tuple_struct: seq()
- Enums are serialized as:
    - unit_variant: variant_index
    - newtype_variant: variant_index + self
    - tuple_variant: variant_index + tuple()
    - struct_variant: variant_index + struct()
- seq(): Sequences are serialized as:
    - SEQ_DELIMITER + value_1 + SEQ_VALUE_DELIMITER + value_2 + SEQ_VALUE_DELIMITER + ... + SEQ_DELIMITER
- map(): Maps are serialized as:
    - key_1 + MAP_KEY_DELIMITER +
      value_1 + MAP_VALUE_DELIMITER +
      key_2 + MAP_KEY_DELIMITER +
      value_2 + MAP_VALUE_DELIMITER +
      ... + MAP_DELIMITER
- Tuples and Structs are serialized as:
    - tuple: seq()
    - struct: map()


### license.

It's MIT so you can do whatever you want. You can still read it [here](./LICENSE.md).
