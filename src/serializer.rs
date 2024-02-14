use serde::{Serialize, Serializer};

use crate::error::CustomError;

pub const NULL: u32 = 0x00;
pub const DAGGER: u32 = 0x86;
pub const DOUBLE_DAGGER: u32 = 0x87;
pub const PIPE: u32 = 0xA6;

#[derive(Debug)]
struct CustomSerializer {
    output: Vec<u8>,
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>, CustomError>
where
    T: Serialize,
{
    let mut serializer = CustomSerializer { output: vec![] };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl CustomSerializer {
    pub fn peek_last(&self, length: usize) -> Result<&[u8], CustomError> {
        match self.output.len() > length {
            true => {
                let last_bytes = self.output.get(self.output.len() - length..).ok_or(
                    CustomError::UnexpectedNone(
                        "attempted to get last 4 bytes but failed".to_string(),
                    ),
                )?;
                Ok(last_bytes)
            }
            false => Err(CustomError::UnexpectedEOF),
        }
    }
}

impl<'a> serde::ser::Serializer for &'a mut CustomSerializer {
    type Ok = ();
    type Error = CustomError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    /// True: High; False: Low
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.output.push(if v { 1 } else { 0 });
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.output.extend(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.output.extend(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.output.extend(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.output.extend(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.output.push(v);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.output.extend(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.output.extend(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.output.extend(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.output.extend(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.output.extend(&v.to_le_bytes());
        Ok(())
    }

    /// 'a'
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        // char is guaranteed to have the same size, alignment, and function call ABI as u32 on all platforms.
        self.serialize_u32(u32::from(v))
    }

    /// "Hello, World!"
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let length = v.len() as u64;
        self.serialize_u64(length)?;
        self.output.extend(v.as_bytes());
        Ok(())
    }

    /// [u8]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let length = v.len() as u64;
        self.serialize_u64(length)?;
        self.output.extend(v);
        Ok(())
    }

    /// None
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    /// Some(T)
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    /// ()
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(NULL)
    }

    /// struct Unit or PhantomData<T>
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    /// struct Millimeters(u8)
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

    /// E::A and E::B in enum E { A, B }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(DAGGER)?;
        self.serialize_u32(variant_index)
    }

    /// E::N in enum E { N(u8) }
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
        self.serialize_u32(DOUBLE_DAGGER)?;
        self.serialize_u32(variant_index)?;
        value.serialize(&mut *self)?;
        self.serialize_u32(DOUBLE_DAGGER)?;
        Ok(())
    }

    /// Vec<T> or HashSet<T>
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.serialize_u32(DOUBLE_DAGGER)?;
        Ok(self)
    }

    /// (u8,) or (String, u64, Vec<T>) or [u64; 10]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    /// struct Rgb(u8, u8, u8)
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    /// E::T in enum E { T(u8, u8) }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_u32(DOUBLE_DAGGER)?;
        self.serialize_u32(variant_index)?;
        Ok(self)
    }

    /// BTreeMap<K, V>
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.serialize_u32(DOUBLE_DAGGER)?;
        Ok(self)
    }

    /// struct S { r: u8, g: u8, b: u8 }
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    /// E::S in enum E { S { r: u8, g: u8, b: u8 } }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_u32(variant_index)?;
        Ok(self)
    }
}

impl<'a> serde::ser::SerializeSeq for &'a mut CustomSerializer {
    type Ok = ();
    type Error = CustomError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        // If the last 4 bytes are not DOUBLE_DAGGER, then add DAGGER.
        // This simply means "don't add DAGGER at the start".
        if self.peek_last(4)? != DOUBLE_DAGGER.to_le_bytes() {
            self.serialize_u32(DAGGER)?;
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(DOUBLE_DAGGER)?;
        Ok(())
    }
}

impl<'a> serde::ser::SerializeTuple for &'a mut CustomSerializer {
    type Ok = ();
    type Error = CustomError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.peek_last(4)? != DOUBLE_DAGGER.to_le_bytes() {
            self.serialize_u32(DAGGER)?;
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(DOUBLE_DAGGER)?;
        Ok(())
    }
}

impl<'a> serde::ser::SerializeTupleStruct for &'a mut CustomSerializer {
    type Ok = ();
    type Error = CustomError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.peek_last(4)? != DOUBLE_DAGGER.to_le_bytes() {
            self.serialize_u32(DAGGER)?;
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(DOUBLE_DAGGER)?;
        Ok(())
    }
}

impl<'a> serde::ser::SerializeTupleVariant for &'a mut CustomSerializer {
    type Ok = ();
    type Error = CustomError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        // we know the last 8 bytes are the the dagger and the variant index
        let last_second_word = u32::from_le_bytes(
            self.peek_last(8)?[0..4]
                .try_into()
                .map_err(|_| CustomError::InvalidTypeSize)?,
        );
        if last_second_word != DOUBLE_DAGGER {
            self.serialize_u32(DAGGER)?;
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(DOUBLE_DAGGER)?;
        todo!()
    }
}

impl<'a> serde::ser::SerializeMap for &'a mut CustomSerializer {
    type Ok = ();
    type Error = CustomError;

    /// DD key | value D key | value D key | value DD
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.peek_last(4)? != DOUBLE_DAGGER.to_le_bytes() {
            self.serialize_u32(DAGGER)?;
        }
        key.serialize(&mut **self)?;
        self.serialize_u32(PIPE)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(DOUBLE_DAGGER)?;
        Ok(())
    }
}

impl<'a> serde::ser::SerializeStruct for &'a mut CustomSerializer {
    type Ok = ();
    type Error = CustomError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.peek_last(4)? != DOUBLE_DAGGER.to_le_bytes() {
            self.serialize_u32(DAGGER)?;
        }
        key.serialize(&mut **self)?;
        self.serialize_u32(PIPE)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(DOUBLE_DAGGER)?;
        Ok(())
    }
}

impl<'a> serde::ser::SerializeStructVariant for &'a mut CustomSerializer {
    type Ok = ();
    type Error = CustomError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
