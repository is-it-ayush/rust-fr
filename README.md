### rust-fr

'rust-fr' aka 'rust for real' is a simple data-interchange format that is better than `serde_json`
but not as awesome as other compact binary formats like [ciborium](https://github.com/enarx/ciborium)
& [msgpack-rust](https://github.com/3Hren/msgpack-rust).

### installation

You can use either of these methods.

- Add via `cargo add rust-fr`
- Add via `Cargo.toml`
```.toml
[dependencies]
rust-fr = "0.1.0"
```

### usage.

```rs
use serde::{Serialize, Deserialize};

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
let human_bytes = rust_fr::protocol::serializer::to_bytes(&human).unwrap();

// deserialize the data from serialized bytes.
let deserialized_human = rust_fr::protocol::deserializer::from_bytes::<Human>(&human_bytes).unwrap();

assert_eq!(human, deserialized_human);
```

### why?

The aim was to learn. I wrote this so I can learn how serde internally works
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
- Delimiters are all serailized as a u8 (1 byte)
- Delimiters Used (the values themselves are arbitrary and could be swapped):
    - STRING_DELIMITER: 0x01
    - BYTE_DELIMITER: 0x02
    - UNIT: 0x03
    - SEQ_DELIMITER: 0x04
    - SEQ_VALUE_DELIMITER: 0x05
    - MAP_DELIMITER: 0x06
    - MAP_KEY_DELIMITER: 0x07
    - MAP_VALUE_DELIMITER: 0x08
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
