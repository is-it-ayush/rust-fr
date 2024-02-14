use crate::{
    error::CustomError,
    serializer::{DAGGER, DOUBLE_DAGGER, NULL, PIPE},
};
use serde::{
    de::{
        self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess,
        Visitor,
    },
    Deserialize, Deserializer,
};

#[derive(Debug)]
pub struct CustomDeserializer<'de> {
    input: &'de [u8],
}

impl<'de> CustomDeserializer<'de> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        CustomDeserializer { input }
    }
}

pub fn from_bytes<'de, T>(input: &'de [u8]) -> Result<T, CustomError>
where
    T: Deserialize<'de>,
{
    let mut deserializer = CustomDeserializer::from_bytes(input);
    let value = T::deserialize(&mut deserializer)?;
    Ok(value)
}

/// Simpler Parser Methods.
impl<'de> CustomDeserializer<'de> {
    /// Returns the next byte in the input without consuming it.
    pub fn peek_byte(&self) -> Result<u8, CustomError> {
        self.input.first().map(|&v| v).ok_or(CustomError::EOF)
    }

    /// Returns the next `length` bytes in the input without consuming them.
    pub fn peek_bytes(&self, length: usize) -> Result<&'de [u8], CustomError> {
        if self.input.len() < length {
            return Err(CustomError::UnexpectedEOF);
        }
        Ok(&self.input[..length])
    }

    /// Returns the next character in the input without consuming it.
    pub fn peek_char(&self) -> Result<u32, CustomError> {
        let bytes = self.peek_bytes(4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// Consumes the next byte in the input.
    pub fn next_byte(&mut self) -> Result<u8, CustomError> {
        let byte = self.peek_byte()?;
        self.input = &self.input[1..];
        Ok(byte)
    }

    /// Consumes the next `length` bytes in the input.
    pub fn next_bytes(&mut self, length: usize) -> Result<&'de [u8], CustomError> {
        if self.input.len() < length {
            return Err(CustomError::UnexpectedEOF);
        }
        let bytes = &self.input[..length];
        self.input = &self.input[length..];
        Ok(bytes)
    }

    /// Parser Methods

    /// Parses a boolean value from the input.
    pub fn parse_bool(&mut self) -> Result<bool, CustomError> {
        Ok(self.next_byte()? != 0)
    }

    /// Parses an unsigned integer value from the input.
    pub fn parse_unsigned<T>(&mut self) -> Result<T, CustomError>
    where
        T: TryFrom<u8> + TryFrom<u16> + TryFrom<u32> + TryFrom<u64>,
    {
        let length = std::mem::size_of::<T>();
        if self.input.len() < length {
            return Err(CustomError::UnexpectedEOF);
        }
        match length {
            1 => {
                let byte = self.next_byte()?;
                u8::from_le_bytes([byte])
                    .try_into()
                    .map_err(|_| CustomError::ConversionError)
            }
            2 => {
                let bytes = self.next_bytes(length)?;
                u16::from_le_bytes([bytes[0], bytes[1]])
                    .try_into()
                    .map_err(|_| CustomError::ConversionError)
            }
            4 => {
                let bytes = self.next_bytes(length)?;
                u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                    .try_into()
                    .map_err(|_| CustomError::ConversionError)
            }
            8 => {
                let bytes = self.next_bytes(length)?;
                u64::from_le_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ])
                .try_into()
                .map_err(|_| CustomError::ConversionError)
            }
            _ => return Err(CustomError::InvalidTypeSize),
        }
    }

    /// Parses a signed integer value from the input.
    pub fn parse_signed<T>(&mut self) -> Result<T, CustomError>
    where
        T: TryFrom<i8> + TryFrom<i16> + TryFrom<i32> + TryFrom<i64>,
    {
        let length = std::mem::size_of::<T>();
        if self.input.len() < length {
            return Err(CustomError::UnexpectedEOF);
        }
        match length {
            1 => {
                let byte = self.next_byte()?;
                i8::from_le_bytes([byte])
                    .try_into()
                    .map_err(|_| CustomError::ConversionError)
            }
            2 => {
                let bytes = self.next_bytes(length)?;
                i16::from_le_bytes([bytes[0], bytes[1]])
                    .try_into()
                    .map_err(|_| CustomError::ConversionError)
            }
            4 => {
                let bytes = self.next_bytes(length)?;
                i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                    .try_into()
                    .map_err(|_| CustomError::ConversionError)
            }
            8 => {
                let bytes = self.next_bytes(length)?;
                i64::from_le_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ])
                .try_into()
                .map_err(|_| CustomError::ConversionError)
            }
            _ => return Err(CustomError::InvalidTypeSize),
        }
    }

    /// Parses a 32-bit floating point value from the input.
    pub fn parse_f32(&mut self) -> Result<f32, CustomError> {
        let bytes = self.next_bytes(4)?;
        Ok(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// Parses a 64-bit floating point value from the input.
    pub fn parse_f64(&mut self) -> Result<f64, CustomError> {
        let bytes = self.next_bytes(8)?;
        Ok(f64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    /// Parses a character value from the input.
    pub fn parse_char(&mut self) -> Result<char, CustomError> {
        let value = self.parse_unsigned::<u32>()?;
        Ok(std::char::from_u32(value).unwrap())
    }

    /// Parses a str value from the input.
    pub fn parse_str(&mut self) -> Result<&'de str, CustomError> {
        let length = self.parse_unsigned::<u64>()? as usize;
        let bytes = self.next_bytes(length)?;
        std::str::from_utf8(bytes).map_err(|_| CustomError::ConversionError)
    }

    /// Parses bytes from the input.
    pub fn parse_bytes(&mut self) -> Result<&'de [u8], CustomError> {
        let length = self.parse_unsigned::<u64>()? as usize;
        self.next_bytes(length)
    }
}

impl<'de, 'a> Deserializer<'de> for &'a mut CustomDeserializer<'de> {
    type Error = CustomError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(CustomError::NotSupported(
            "call to 'deserialize_any' are unsupported".to_string(),
        ))
    }

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

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_str(self.parse_str()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_string(self.parse_str()?.to_string())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_bytes(self.parse_bytes()?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_byte_buf(self.parse_bytes()?.to_vec())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.parse_unsigned::<u32>()? {
            NULL => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.parse_unsigned::<u32>()? {
            NULL => visitor.visit_unit(),
            _ => Err(CustomError::ExpectedNull),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.parse_unsigned::<u32>()? {
            DOUBLE_DAGGER => {
                let value = visitor.visit_seq(SequenceVistor::new(self))?;
                match self.peek_char()? {
                    DOUBLE_DAGGER => Ok(value),
                    _ => Err(CustomError::ExpectedSequenceEnd),
                }
            }
            _ => Err(CustomError::ExpectedSequenceStart),
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    // DD key | value D key | value D key | value DD
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.parse_unsigned::<u32>()? {
            DOUBLE_DAGGER => {
                let value = visitor.visit_map(MapVistor::new(self))?;
                match self.peek_char()? {
                    DOUBLE_DAGGER => Ok(value),
                    _ => Err(CustomError::ExpectedMapEnd),
                }
            }
            _ => Err(CustomError::ExpectedMapEnd),
        }
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.parse_signed::<u32>()? {
            // newtype variant
            DOUBLE_DAGGER => {
                let value = visitor.visit_enum(EnumVistor::new(self))?;
                match self.peek_char()? {
                    DOUBLE_DAGGER => Ok(value),
                    _ => Err(CustomError::ExpectedEnum),
                }
            }
            // unit variant
            DAGGER => visitor.visit_enum(self.parse_unsigned::<u32>()?.into_deserializer()),
            _ => Err(CustomError::ExpectedEnum),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(CustomError::NotSupported(
            "call to 'deserialize_ignored_any' are unsupported".to_string(),
        ))
    }
}

struct SequenceVistor<'a, 'de: 'a> {
    deserializer: &'a mut CustomDeserializer<'de>,
    first: bool,
}

impl<'a, 'de> SequenceVistor<'a, 'de> {
    fn new(deserializer: &'a mut CustomDeserializer<'de>) -> Self {
        SequenceVistor {
            deserializer,
            first: true,
        }
    }
}

impl<'de, 'a> SeqAccess<'de> for SequenceVistor<'a, 'de> {
    type Error = CustomError;

    /// ‡a†b†c†d†e‡
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.deserializer.peek_char()? == DOUBLE_DAGGER {
            return Ok(None);
        }
        // not first and not dagger; throw
        if !self.first && self.deserializer.parse_unsigned::<u32>()? != DAGGER {
            return Err(CustomError::ExpectedSequenceEnd);
        }
        self.first = false;
        seed.deserialize(&mut *self.deserializer).map(Some)
    }
}

struct MapVistor<'a, 'de: 'a> {
    deserializer: &'a mut CustomDeserializer<'de>,
    first: bool,
}

impl<'a, 'de> MapVistor<'a, 'de> {
    fn new(deserializer: &'a mut CustomDeserializer<'de>) -> Self {
        MapVistor {
            deserializer,
            first: true,
        }
    }
}

impl<'de, 'a> MapAccess<'de> for MapVistor<'a, 'de> {
    type Error = CustomError;

    /// key | value D key | value D key | value DD
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.deserializer.peek_char()? == DOUBLE_DAGGER {
            return Ok(None);
        }
        // not first and not dagger; throw
        if !self.first && self.deserializer.parse_unsigned::<u32>()? != DAGGER {
            return Err(CustomError::ExpectedSequenceEnd);
        }
        self.first = false;
        seed.deserialize(&mut *self.deserializer).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        if self.deserializer.parse_unsigned::<u32>()? != PIPE {
            return Err(CustomError::ExpectedPipe);
        }
        seed.deserialize(&mut *self.deserializer)
    }
}

struct EnumVistor<'a, 'de: 'a> {
    deserializer: &'a mut CustomDeserializer<'de>,
}

impl<'a, 'de> EnumVistor<'a, 'de> {
    fn new(deserializer: &'a mut CustomDeserializer<'de>) -> Self {
        EnumVistor { deserializer }
    }
}

impl<'de, 'a> EnumAccess<'de> for EnumVistor<'a, 'de> {
    type Error = CustomError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let key = self.deserializer.parse_unsigned::<u32>()?;
        Ok((seed.deserialize(key.into_deserializer())?, self))
    }
}

impl<'de, 'a> VariantAccess<'de> for EnumVistor<'a, 'de> {
    type Error = CustomError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Err(CustomError::ExpectedU32)
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(&mut *self.deserializer, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_map(&mut *self.deserializer, visitor)
    }
}
