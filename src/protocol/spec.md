#### format specification.
- Primitive types are serialized as is.
    - bool: 0 -> false, 1 -> true (1 byte)
    - i8, i16, i32, i64: as is.
    - u8, u16, u32, u64: as is.
    - f32, f64: as is.
    - char: as u32 (4 bytes)
- Delimiters are all serailized as a u8 (1 byte)
- Delimiters Used:
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
