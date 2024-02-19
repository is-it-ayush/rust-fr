#### specification.
- The seperators are u8.
- The seperators need to be unique among serde-data-model types.
- Primitive types are serialized as is.
    - bool: 0 -> false, 1 -> true (1 byte)
    - i8, i16, i32, i64: as is.
    - u8, u16, u32, u64: as is.
    - f32, f64: as is.
    - char: as u32 (4 bytes)

#### Need delimiters here because if I encode length then I'd waste 64 bit space. : )
- String, Bytes, Unit, Option are serialized as:
    - str: STRING_DELIMITER + bytes + STRING_DELIMITER
    - bytes: BYTE_DELIMITER + bytes + BYTE_DELIMITER
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

- Sequences are serialized as:
    - SEQ_DELIMITER + value_1 + SEQ_VALUE_DELIMITER + value_2 + SEQ_VALUE_DELIMITER + ... + SEQ_DELIMITER

- Maps are serialized as:
    - MAP_DELIMITER +
        key_1 + MAP_KEY_DELIMITER +
        value_1 + MAP_VALUE_DELIMITER +
        key_2 + MAP_KEY_DELIMITER +
        value_2 + MAP_VALUE_DELIMITER +
        ...
    MAP_DELIMITER

- Tuples and Structs are serialized as:
    - tuple: seq()
    - struct: map()
