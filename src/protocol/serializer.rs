use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};

use super::error::Error;

pub const STRING_DELIMITER: u8 = 0x22; // " (double quote)
pub const BYTE_DELIMITER: u8 = 0x23; // # (hash)
pub const UNIT: u8 = 0x05; // ENQ (enquiry)
pub const SEQ_DELIMITER: u8 = 0x26; // & (ampersand)
pub const SEQ_VALUE_DELIMITER: u8 = 0x2E; // . (period)
pub const MAP_DELIMITER: u8 = 0x3A; // : (colon)
pub const MAP_KEY_DELIMITER: u8 = 0x3B; // ; (semicolon)
pub const MAP_VALUE_DELIMITER: u8 = 0x3C; // < (less than)

#[derive(Debug)]
struct CustomSerializer {
    data: Vec<u8>,
}

pub fn to_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    let mut serializer = CustomSerializer { data: Vec::new() };
    value.serialize(&mut serializer)?;
    Ok(serializer.data)
}

impl CustomSerializer {
    /// Get the last byte from the data.
    pub fn peek_byte(&self) -> Result<&u8, Error> {
        self.data.last().ok_or(Error::NoByte)
    }

    /// Get the last 'n' bytes from the data.
    pub fn peek_bytes(&self, n: usize) -> Result<&[u8], Error> {
        let len = self.data.len();
        if len < n {
            return Err(Error::NLargerThanLength(n, len));
        }
        Ok(&self.data[len - n..])
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

    /// bool: 0 -> false, 1 -> true (1 byte)
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.data.push(if v { 1 } else { 0 });
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
    /// str: STRING_DELIMITER bytes STRING_DELIMITER
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(STRING_DELIMITER)?;
        self.data.extend(v.as_bytes());
        self.serialize_u8(STRING_DELIMITER)?;
        Ok(())
    }
    /// bytes: BYTE_DELIMITER bytes BYTE_DELIMITER
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(BYTE_DELIMITER)?;
        self.data.extend(v);
        self.serialize_u8(BYTE_DELIMITER)?;
        Ok(())
    }

    /// unit: UNIT (null)
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(UNIT)
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

    /// sequences: SEQ_DELIMITER value_1 SEQ_VALUE_DELIMITER value_2 SEQ_VALUE_DELIMITER ... SEQ_DELIMITER
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.serialize_u8(SEQ_DELIMITER)?;
        Ok(self)
    }
    /// maps: MAP_DELIMITER key_1 MAP_KEY_DELIMITER value_1 MAP_VALUE_DELIMITER key_2 MAP_KEY_DELIMITER value_2 MAP_VALUE_DELIMITER ... MAP_DELIMITER
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

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.peek_byte()? != &SEQ_DELIMITER {
            self.serialize_u8(SEQ_VALUE_DELIMITER)?;
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(SEQ_DELIMITER)
    }
}
impl<'a> SerializeMap for &'a mut CustomSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)?;
        self.serialize_u8(MAP_KEY_DELIMITER)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)?;
        self.serialize_u8(MAP_VALUE_DELIMITER)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(MAP_DELIMITER)
    }
}

// = seq()
impl<'a> SerializeTuple for &'a mut CustomSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.peek_byte()? != &SEQ_DELIMITER {
            self.serialize_u8(SEQ_VALUE_DELIMITER)?;
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(SEQ_DELIMITER)
    }
}
// = map()
impl<'a> SerializeStruct for &'a mut CustomSerializer {
    type Ok = ();
    type Error = Error;

    // MAP_DELIMITER + MAP_KEY_DELIMITER + key + MAP_KEY_DELIMITER + MAP_VALUE_DELIMITER + value + MAP_VALUE_DELIMITER + ... + MAP_DELIMITER
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)?;
        self.serialize_u8(MAP_KEY_DELIMITER)?;
        value.serialize(&mut **self)?;
        self.serialize_u8(MAP_VALUE_DELIMITER)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(MAP_DELIMITER)
    }
}

// = seq()
impl<'a> SerializeTupleStruct for &'a mut CustomSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.peek_byte()? != &SEQ_DELIMITER {
            self.serialize_u8(SEQ_VALUE_DELIMITER)?;
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(SEQ_DELIMITER)
    }
}

// = tuple() = seq()
impl<'a> SerializeTupleVariant for &'a mut CustomSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.peek_bytes(5)?[0] != SEQ_DELIMITER {
            self.serialize_u8(SEQ_VALUE_DELIMITER)?;
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(SEQ_DELIMITER)
    }
}

// = struct() = map()
impl<'a> SerializeStructVariant for &'a mut CustomSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)?;
        self.serialize_u8(MAP_KEY_DELIMITER)?;
        value.serialize(&mut **self)?;
        self.serialize_u8(MAP_VALUE_DELIMITER)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(MAP_DELIMITER)
    }
}
