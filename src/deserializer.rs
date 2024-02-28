//! ### Deserializer
//! This module contains the deserialization logic for the library. It is used to deserialize
//! bytes to a custom type.
//!
//! To use the deserializer, you need to call the [`from_bytes`] function which takes in
//! the bytes and a type. The type must implement the `Deserialize` trait from the serde library.
//! It returns a Result with the deserialized data or an error.

use bitvec::{prelude as bv, slice::BitSlice, view::BitView};
use serde::{
    de::{EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess},
    Deserialize, Deserializer,
};

use super::{error::Error, serializer::Delimiter};

// Internal struct that handles the deserialization of the data.
// It has a few methods that allows us to peek and eat bytes from the data.
// It also has methods to parse some data into the required type.
#[derive(Debug)]
struct CustomDeserializer<'de> {
    data: &'de bv::BitSlice<u8, bv::Lsb0>,
}

/// The function to deserialize (serialized) bytes back into data. `T` must implement the `Deserialize` trait
/// from the `serde` library. `bytes` is the data to be deserialized. It returns a Result with the deserialized
/// data or an error.
pub fn from_bytes<'de, T>(bytes: &'de [u8]) -> Result<T, Error>
where
    T: Deserialize<'de>,
{
    let mut deserializer = CustomDeserializer {
        data: bytes.view_bits(),
    };
    let deserialized = T::deserialize(&mut deserializer)?;
    Ok(deserialized)
}

impl<'de> CustomDeserializer<'de> {
    /// Get 'n' bits from end of the data.
    /// Example: If the data is 0b10101010 and n is 3, the result will be 0b010.
    fn _peek_n_bits(&self, size: usize) -> Result<&BitSlice<u8>, Error> {
        let len = self.data.len();
        if size > len {
            return Err(Error::NLargerThanLength(size, self.data.len()));
        }
        self.data.get(..size).ok_or(Error::NoByte)
    }

    /// Get the first byte from the data.
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

    /// Peek the next token from the data.
    pub fn peek_token(&self, token: Delimiter) -> Result<bool, Error> {
        let bits = match token {
            Delimiter::String => self._peek_n_bits(8)?,
            Delimiter::Byte => self._peek_n_bits(8)?,
            Delimiter::Map => self._peek_n_bits(8)?,
            _ => self._peek_n_bits(3)?,
        };
        let mut byte = 0u8;
        for (i, bit) in bits.iter().enumerate() {
            if *bit {
                byte |= 1 << i;
            }
        }
        if byte == token as u8 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Grab the next bit from the data and remove it.
    pub fn eat_bit(&mut self) -> Result<bool, Error> {
        let bit = *self._peek_n_bits(1)?.get(0).ok_or(Error::NoBit)?;
        self.data = &self.data[1..];
        Ok(bit)
    }

    /// Grab the next byte from the data and remove it.
    pub fn eat_byte(&mut self) -> Result<u8, Error> {
        let byte = self.peek_byte()?;
        self.data = &self.data[8..];
        Ok(byte)
    }

    /// Grab the next 'n' bytes from the data and remove them.
    pub fn eat_bytes(&mut self, n: usize) -> Result<Vec<u8>, Error> {
        let bits = &self.data[..n * 8];
        let mut bytes = Vec::new();
        self.data = &self.data[n * 8..];
        for i in 0..n {
            let mut byte = 0u8;
            for (j, bit) in bits[i * 8..(i + 1) * 8].iter().enumerate() {
                if *bit {
                    byte |= 1 << j;
                }
            }
            bytes.push(byte);
        }
        Ok(bytes)
    }

    /// Grab the next token from the data and remove it.
    pub fn eat_token(&mut self, token: Delimiter) -> Result<(), Error> {
        let bits_to_munch = match token {
            Delimiter::String => 8,
            Delimiter::Byte => 8,
            Delimiter::Map => 8,
            _ => 3,
        };
        if self.data.len() < bits_to_munch {
            return Err(Error::UnexpectedEOF);
        }
        self.data = &self.data[bits_to_munch..];
        Ok(())
    }

    /// Parser Methods

    /// Parses a boolean value from the input.
    pub fn parse_bool(&mut self) -> Result<bool, Error> {
        self.eat_bit()
    }
    /// Parses an unsigned integer value from the input.
    pub fn parse_unsigned<T>(&mut self) -> Result<T, Error>
    where
        T: TryFrom<u8> + TryFrom<u16> + TryFrom<u32> + TryFrom<u64>,
    {
        let length = std::mem::size_of::<T>();
        if self.data.len() < length {
            return Err(Error::UnexpectedEOF);
        }
        match length {
            1 => {
                let byte = self.eat_byte()?;
                u8::from_le_bytes([byte])
                    .try_into()
                    .map_err(|_| Error::ConversionError)
            }
            2 => {
                let bytes = self.eat_bytes(length)?;
                u16::from_le_bytes([bytes[0], bytes[1]])
                    .try_into()
                    .map_err(|_| Error::ConversionError)
            }
            4 => {
                let bytes = self.eat_bytes(length)?;
                u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                    .try_into()
                    .map_err(|_| Error::ConversionError)
            }
            8 => {
                let bytes = self.eat_bytes(length)?;
                u64::from_le_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ])
                .try_into()
                .map_err(|_| Error::ConversionError)
            }
            _ => Err(Error::InvalidTypeSize),
        }
    }
    /// Parses a signed integer value from the input.
    pub fn parse_signed<T>(&mut self) -> Result<T, Error>
    where
        T: TryFrom<i8> + TryFrom<i16> + TryFrom<i32> + TryFrom<i64>,
    {
        let length = std::mem::size_of::<T>();
        if self.data.len() < length {
            return Err(Error::UnexpectedEOF);
        }
        match length {
            1 => {
                let byte = self.eat_byte()?;
                i8::from_le_bytes([byte])
                    .try_into()
                    .map_err(|_| Error::ConversionError)
            }
            2 => {
                let bytes = self.eat_bytes(length)?;
                i16::from_le_bytes([bytes[0], bytes[1]])
                    .try_into()
                    .map_err(|_| Error::ConversionError)
            }
            4 => {
                let bytes = self.eat_bytes(length)?;
                i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                    .try_into()
                    .map_err(|_| Error::ConversionError)
            }
            8 => {
                let bytes = self.eat_bytes(length)?;
                i64::from_le_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ])
                .try_into()
                .map_err(|_| Error::ConversionError)
            }
            _ => Err(Error::InvalidTypeSize),
        }
    }
    /// Parses a 32-bit floating point value from the input.
    pub fn parse_f32(&mut self) -> Result<f32, Error> {
        let bytes = self.eat_bytes(4)?;
        Ok(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
    /// Parses a 64-bit floating point value from the input.
    pub fn parse_f64(&mut self) -> Result<f64, Error> {
        let bytes = self.eat_bytes(8)?;
        Ok(f64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }
    /// Parses a character value from the input.
    pub fn parse_char(&mut self) -> Result<char, Error> {
        let value = self.parse_unsigned::<u32>()?;
        Ok(std::char::from_u32(value).unwrap())
    }

    /// Parses a string value from the input.
    pub fn parse_str(&mut self, bytes: &mut Vec<u8>) -> Result<String, Error> {
        'byteloop: loop {
            let byte = self.eat_byte()?;
            bytes.push(byte);
            if self.peek_token(Delimiter::String)? {
                self.eat_token(Delimiter::String)?;
                break 'byteloop;
            }
        }
        String::from_utf8(bytes.clone()).map_err(|_| Error::ConversionError)
    }

    /// Parses a byte buffer from the input.
    pub fn parse_bytes(&mut self, bytes: &mut Vec<u8>) -> Result<(), Error> {
        loop {
            if self.peek_token(Delimiter::Byte)? {
                self.eat_token(Delimiter::Byte)?;
                break;
            }
            let byte = self.eat_byte()?;
            bytes.push(byte);
        }
        Ok(())
    }
}

impl<'de, 'a> Deserializer<'de> for &'a mut CustomDeserializer<'de> {
    type Error = Error;

    /// The data is not self-describing, so we need to use the type to determine how to deserialize it.
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedCall("deserialize_any".to_string()))
    }

    // Primitve Types Deserialization. They are serialized as is (LE byte order).

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i8(self.parse_signed::<i8>()?)
    }
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i16(self.parse_signed::<i16>()?)
    }
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i32(self.parse_signed::<i32>()?)
    }
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i64(self.parse_signed::<i64>()?)
    }
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u8(self.parse_unsigned::<u8>()?)
    }
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u16(self.parse_unsigned::<u16>()?)
    }
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u32(self.parse_unsigned::<u32>()?)
    }
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u64(self.parse_unsigned::<u64>()?)
    }
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f32(self.parse_f32()?)
    }
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f64(self.parse_f64()?)
    }
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_char(self.parse_char()?)
    }

    /// String Deserialization. They are serialized as bytes + STRING_DELIMITER.
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut bytes = Vec::new();
        visitor.visit_str(self.parse_str(&mut bytes)?.as_str())
    }
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut bytes = Vec::new();
        visitor.visit_string(self.parse_str(&mut bytes)?.to_string())
    }

    /// Byte Deserialization. They are serialized as bytes + BYTE_DELIMITER.
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut bytes = Vec::new();
        self.parse_bytes(&mut bytes)?;
        visitor.visit_bytes(&bytes)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut bytes = Vec::new();
        self.parse_bytes(&mut bytes)?;
        visitor.visit_byte_buf(bytes)
    }

    /// Option Deserialization. They are serialized as None -> unit(), Some -> self.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.peek_token(Delimiter::Unit)? {
            true => {
                self.eat_token(Delimiter::Unit)?;
                visitor.visit_none()
            }
            false => visitor.visit_some(self),
        }
    }
    /// Unit Deserialization. They are serialized as UNIT.
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.peek_token(Delimiter::Unit)? {
            true => {
                self.eat_token(Delimiter::Unit)?;
                visitor.visit_unit()
            }
            _ => Err(Error::ExpectedDelimiter(Delimiter::Unit)),
        }
    }

    /// Struct Deserialization.
    /// - unit_struct: unit()
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }
    /// - newtype_struct: self
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }
    /// - tuple_struct: seq()
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    /// Enum Deserialization.
    /// - unit_variant: ENUM_DELIMITER + variant_index
    /// - newtype_variant: ENUM_DELIMITER + variant_index + self
    /// - tuple_variant: ENUM_DELIMITER + variant_index + tuple()
    /// - struct_variant: ENUM_DELIMITER + variant_index + struct()
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    /// Seq & Map Deserialization.
    /// - seq: SEQ_DELIMITER + value_1 + SEQ_VALUE_DELIMITER + value_2 + SEQ_VALUE_DELIMITER + ... + SEQ_DELIMITER
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.peek_token(Delimiter::Seq)? {
            true => {
                self.eat_token(Delimiter::Seq)?;
                let value = visitor.visit_seq(SequenceDeserializer::new(self))?;
                if !self.peek_token(Delimiter::Seq)? {
                    return Err(Error::ExpectedDelimiter(Delimiter::Seq));
                }
                self.eat_token(Delimiter::Seq)?;
                Ok(value)
            }
            false => Err(Error::ExpectedDelimiter(Delimiter::Seq)),
        }
    }
    /// - map: key_1 + MAP_KEY_DELIMITER + value_1 + MAP_VALUE_DELIMITER + ... + MAP_DELIMITER
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let value = visitor.visit_map(MapDeserializer::new(self))?;
        if !self.peek_token(Delimiter::Map)? {
            return Err(Error::ExpectedDelimiter(Delimiter::Map));
        }
        self.eat_token(Delimiter::Map)?;
        Ok(value)
    }

    /// Tuple & Struct Deserialization.
    /// - tuple: seq()
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }
    /// - struct: map()
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedCall(
            "deserialize_ignored_any".to_string(),
        ))
    }
}

/// Handles the deserialization of an enum.
/// enum() => variant_index + (depends on variant type; handled by VARIANT_ACCESS)
impl<'de, 'a> EnumAccess<'de> for &'a mut CustomDeserializer<'de> {
    type Error = Error;
    type Variant = Self;

    /// Get the next variant key from the data and remove it.
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let key = self.parse_unsigned::<u32>()?;
        Ok((seed.deserialize(key.into_deserializer())?, self))
    }
}
impl<'de, 'a> VariantAccess<'de> for &'a mut CustomDeserializer<'de> {
    type Error = Error;

    /// - unit_variant: variant_index
    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// - newtype_variant: variant_index + self
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }

    /// - tuple_variant: variant_index + tuple() where (tuple() => seq())
    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    /// - struct_variant: variant_index + struct() where (struct() => map())
    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_struct("", fields, visitor)
    }
}

/// Internal struct that handles the deserialization of a sequence.
/// seq() => SEQ_DELIMITER + value_1 + SEQ_VALUE_DELIMITER + value_2 + SEQ_VALUE_DELIMITER + ... + SEQ_DELIMITER
struct SequenceDeserializer<'a, 'de: 'a> {
    deserializer: &'a mut CustomDeserializer<'de>,
    first: bool,
}
impl<'a, 'de> SequenceDeserializer<'a, 'de> {
    pub fn new(deserializer: &'a mut CustomDeserializer<'de>) -> Self {
        Self {
            deserializer,
            first: true,
        }
    }
}
impl<'de, 'a> SeqAccess<'de> for SequenceDeserializer<'a, 'de> {
    type Error = Error;

    /// Grab the next element from the data and remove it.
    /// - If at end of sequence; exit.
    /// - If not first and not at the end of sequence; eat SEQ_VALUE_DELIMITER.
    /// - Make not first; deserialize next element.
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        // if at end of sequence; exit
        if self.deserializer.peek_token(Delimiter::Seq)? {
            return Ok(None);
        }
        // if not first and not at the end of sequence; eat SEQ_VALUE_DELIMITER
        if !self.first {
            if !self.deserializer.peek_token(Delimiter::SeqValue)? {
                return Err(Error::ExpectedDelimiter(Delimiter::SeqValue));
            }
            self.deserializer.eat_token(Delimiter::SeqValue)?;
        }
        // make not first; deserialize next element
        self.first = false;
        seed.deserialize(&mut *self.deserializer).map(Some)
    }
}

/// Internal struct that handles the deserialization of a map.
/// map() => key_1 + MAP_KEY_DELIMITER + value_1 + MAP_VALUE_DELIMITER + ... + MAP_DELIMITER
struct MapDeserializer<'a, 'de: 'a> {
    deserializer: &'a mut CustomDeserializer<'de>,
    first: bool,
}
impl<'a, 'de> MapDeserializer<'a, 'de> {
    pub fn new(deserializer: &'a mut CustomDeserializer<'de>) -> Self {
        Self {
            deserializer,
            first: true,
        }
    }
}
impl<'de, 'a> MapAccess<'de> for MapDeserializer<'a, 'de> {
    type Error = Error;

    /// Grab the next key from the data and remove it.
    /// - If at end of map; exit.
    /// - Make not first; deserialize next key_1.
    /// - Deserialize next value.
    /// - Eat MAP_KEY_DELIMITER.
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        // if at end of map; exit
        if self.deserializer.peek_token(Delimiter::Map)? {
            return Ok(None);
        }
        // make not first; deserialize next key_1
        self.first = false;
        let value = seed.deserialize(&mut *self.deserializer).map(Some)?;
        if !self.deserializer.peek_token(Delimiter::MapKey)? {
            return Err(Error::ExpectedDelimiter(Delimiter::MapKey));
        }
        self.deserializer.eat_token(Delimiter::MapKey)?;
        Ok(value)
    }

    /// Grab the next value from the data and remove it.
    /// - Deserialize next value.
    /// - Eat MAP_VALUE_DELIMITER.
    /// - Return value.
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let value = seed.deserialize(&mut *self.deserializer)?;
        if !self.deserializer.peek_token(Delimiter::MapValue)? {
            return Err(Error::ExpectedDelimiter(Delimiter::MapValue));
        }
        self.deserializer.eat_token(Delimiter::MapValue)?;
        Ok(value)
    }
}
