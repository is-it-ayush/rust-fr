use bitvec::{prelude as bv, slice::BitSlice};
use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};

use super::error::Error;

/// The following constants are used to serialize the data in a specific format.
/// Their exact values are not important, but they should be unique and not conflict with the data.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Delimiter {
    /// STRING_DELIMITER: 0b10000110
    String = 134,
    /// BYTE_DELIMITER: 0b10000111
    Byte = 135,
    /// UNIT: 0b010
    Unit = 2,
    /// SEQ_DELIMITER: 0b011
    Seq = 3,
    /// SEQ_VALUE_DELIMITER: 0b100
    SeqValue = 4,
    /// MAP_DELIMITER: 0b101
    Map = 5,
    /// MAP_KEY_DELIMITER: 0b110
    MapKey = 6,
    /// MAP_VALUE_DELIMITER: 0b111
    MapValue = 7,
}

impl std::fmt::Display for Delimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Delimiter::String => write!(f, "String"),
            Delimiter::Byte => write!(f, "Byte"),
            Delimiter::Unit => write!(f, "Unit"),
            Delimiter::Seq => write!(f, "Seq"),
            Delimiter::SeqValue => write!(f, "SeqValue"),
            Delimiter::Map => write!(f, "Map"),
            Delimiter::MapKey => write!(f, "MapKey"),
            Delimiter::MapValue => write!(f, "MapValue"),
        }
    }
}

/// Internal struct that handles the serialization of the data.
/// It has a few methods that lets us peeking bytes in the data.
#[derive(Debug)]
struct CustomSerializer {
    data: bv::BitVec<u8, bv::Lsb0>,
}

/// The main function to serialize data of a given type to a byte vector i.e. Vec<u8>. It
/// uses the format specification to serialize the data. In order to serialize a custom type,
/// the type must implement the Serialize trait from the serde library.
pub fn to_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    let mut serializer = CustomSerializer {
        data: bv::BitVec::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.data.into_vec())
}

impl CustomSerializer {
    /// Get 'n' bits from end of the data.
    /// Example: If the data is 0b10101010 and n is 3, the result will be 0b010.
    fn _peek_n_bits(&self, size: usize) -> Result<&BitSlice<u8>, Error> {
        let len = self.data.len();
        if size > len {
            return Err(Error::NLargerThanLength(size, self.data.len()));
        }
        self.data.get(len - size..).ok_or(Error::NoByte)
    }

    // Construct a byte from the last 8 bits of the data.
    pub fn peek_byte(&self) -> Result<u8, Error> {
        let bits = self._peek_n_bits(8)?;
        let mut byte = 0u8;
        for (i, bit) in bits.iter().enumerate() {
            if *bit {
                byte |= 1 << i;
            }
        }
        Ok(byte)
    }

    /// Construst a byte from the last 3 bits of the data.
    pub fn peek_token(&self, token: Delimiter) -> Result<bool, Error> {
        let bits = match token {
            Delimiter::String => self._peek_n_bits(8)?,
            Delimiter::Byte => self._peek_n_bits(8)?,
            _ => self._peek_n_bits(3)?,
        };
        let mut byte = 0u8;
        for (i, bit) in bits.iter().enumerate() {
            if *bit {
                byte |= 1 << i;
            }
        }
        Ok(byte == token as u8)
    }

    /// Get token before 'n' bits.
    pub fn peek_token_before_n_bits(&self, n: usize) -> Result<u8, Error> {
        let bits = self._peek_n_bits(n + 3)?[0..3].as_ref();
        let mut byte = 0u8;
        for (i, bit) in bits.iter().enumerate() {
            if *bit {
                byte |= 1 << i;
            }
        }
        Ok(byte)
    }

    /// Serialize a token to the data.
    pub fn serialize_token(&mut self, token: Delimiter) -> () {
        match token {
            Delimiter::String => {
                self.data
                    .extend(&[false, true, true, false, false, false, false, true]);
                // 10000110
            }
            Delimiter::Byte => {
                self.data
                    .extend(&[true, true, true, false, false, false, false, true]);
                // 10000111
            }
            Delimiter::Unit => {
                self.data.extend(&[false, true, false]); // 010
            }
            Delimiter::Seq => {
                self.data.extend(&[true, true, false]); // 011
            }
            Delimiter::SeqValue => {
                self.data.extend(&[false, false, true]); // 100
            }
            Delimiter::Map => {
                self.data.extend(&[true, false, true]); // 101
            }
            Delimiter::MapKey => {
                self.data.extend(&[false, true, true]); // 110
            }
            Delimiter::MapValue => {
                self.data.extend(&[true, true, true]); // 111
            }
        }
    }
}

impl<'a> Serializer for &'a mut CustomSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeMap = Self;

    type SerializeTuple = Self;
    type SerializeStruct = Self;

    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeStructVariant = Self;

    /// bool: 0 -> false, 1 -> true (1 bit)
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.data.push(v);
        Ok(())
    }

    /// i8, i16, i32, i64: Little Endian (1, 2, 4, 8 bytes)
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.data.extend(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.data.extend(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.data.extend(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.data.extend(&v.to_le_bytes());
        Ok(())
    }

    /// u8, u16, u32, u64: Little Endian (1, 2, 4, 8 bytes)
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.data.extend(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.data.extend(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.data.extend(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.data.extend(&v.to_le_bytes());
        Ok(())
    }

    /// f32, f64: Little Endian (4, 8 bytes)
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.data.extend(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.data.extend(&v.to_le_bytes());
        Ok(())
    }

    /// char: as u32 (4 bytes)
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(u32::from(v))
    }
    /// str: bytes STRING_DELIMITER
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.data.extend(v.as_bytes());
        self.serialize_token(Delimiter::String);
        Ok(())
    }
    /// bytes: bytes BYTE_DELIMITER
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.data.extend(v);
        self.serialize_token(Delimiter::Byte);
        Ok(())
    }

    /// unit: UNIT (null)
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_token(Delimiter::Unit);
        Ok(())
    }

    /// option:
    /// None -> unit()
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }
    /// Some -> self
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    /// structs:
    /// unit_struct: unit()
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }
    /// newtype_struct: self
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }
    /// tuple_struct: tuple()
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_tuple(len)
    }

    /// enum:
    /// unit_variant: variant_index
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(variant_index)
    }
    /// newtype_variant: variant_index self
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.serialize_u32(variant_index)?;
        value.serialize(self)
    }
    /// tuple_variant: variant_index tuple()
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_u32(variant_index)?;
        self.serialize_seq(Some(len))
    }
    /// struct_variant: variant_index struct()
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_u32(variant_index)?;
        self.serialize_map(Some(len))
    }

    /// sequences: SEQ_DELIMITER + value_1 + SEQ_VALUE_DELIMITER + value_2 + SEQ_VALUE_DELIMITER + ... SEQ_DELIMITER
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.serialize_token(Delimiter::Seq);
        Ok(self)
    }
    /// maps: key_1 + MAP_KEY_DELIMITER + value_1 + MAP_VALUE_DELIMITER + key_2 + MAP_KEY_DELIMITER + value_2 + MAP_VALUE_DELIMITER +... MAP_DELIMITER
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(self)
    }

    /// tuples: seq()
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }
    /// structs: map()
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }
}

impl<'a> SerializeSeq for &'a mut CustomSerializer {
    type Ok = ();
    type Error = Error;

    /// Serialize an element of the sequence.
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if !self.peek_token(Delimiter::Seq)? {
            self.serialize_token(Delimiter::SeqValue);
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_token(Delimiter::Seq);
        Ok(())
    }
}
impl<'a> SerializeMap for &'a mut CustomSerializer {
    type Ok = ();
    type Error = Error;

    /// Serialize a key of a given element of the map.
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)?;
        self.serialize_token(Delimiter::MapKey);
        Ok(())
    }

    /// Serialize a value of a given element of the map.
    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)?;
        self.serialize_token(Delimiter::MapValue);
        Ok(())
    }

    /// End the map serialization.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_token(Delimiter::Map);
        Ok(())
    }
}

// = seq()
impl<'a> SerializeTuple for &'a mut CustomSerializer {
    type Ok = ();
    type Error = Error;

    /// Serialize an element of the tuple.
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if !self.peek_token(Delimiter::Seq)? {
            self.serialize_token(Delimiter::SeqValue);
        }
        value.serialize(&mut **self)
    }

    /// End the tuple serialization.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_token(Delimiter::Seq);
        Ok(())
    }
}
// = map()
impl<'a> SerializeStruct for &'a mut CustomSerializer {
    type Ok = ();
    type Error = Error;

    /// Serialize a field of the struct. Structs treated as a key-value pair i.e. a map.
    /// There is no difference between a struct and a map in the serialization format.
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)?;
        self.serialize_token(Delimiter::MapKey);
        value.serialize(&mut **self)?;
        self.serialize_token(Delimiter::MapValue);
        Ok(())
    }

    /// End the struct serialization.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_token(Delimiter::Map);
        Ok(())
    }
}

// = seq()
impl<'a> SerializeTupleStruct for &'a mut CustomSerializer {
    type Ok = ();
    type Error = Error;

    /// Serialize an element of the tuple. Tuple structs treated as a sequence.
    /// There is no difference between a tuple struct and a sequence in the serialization format.
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if !self.peek_token(Delimiter::Seq)? {
            self.serialize_token(Delimiter::SeqValue);
        }
        value.serialize(&mut **self)
    }

    /// End the tuple struct serialization.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_token(Delimiter::Seq);
        Ok(())
    }
}

// = tuple() = seq()
impl<'a> SerializeTupleVariant for &'a mut CustomSerializer {
    type Ok = ();
    type Error = Error;

    /// Serialize an element of the tuple in an enum variant. Tuple variants treated as a sequence.
    /// There is no difference between a tuple variant and a sequence in the serialization format.
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.peek_token_before_n_bits(32)? != Delimiter::Seq as u8 {
            self.serialize_token(Delimiter::SeqValue);
        }
        value.serialize(&mut **self)
    }

    /// End the tuple variant serialization.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_token(Delimiter::Seq);
        Ok(())
    }
}

// = struct() = map()
impl<'a> SerializeStructVariant for &'a mut CustomSerializer {
    type Ok = ();
    type Error = Error;

    /// Serialize a field of the struct in an enum variant. Struct variants treated as a key-value pair i.e. a map.
    /// There is no difference between a struct variant and a map in the serialization format.
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)?;
        self.serialize_token(Delimiter::MapKey);
        value.serialize(&mut **self)?;
        self.serialize_token(Delimiter::MapValue);
        Ok(())
    }

    /// End the struct variant serialization.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_token(Delimiter::Map);
        Ok(())
    }
}
