use serde::ser::{Impossible, Serialize, Serializer};

use error::Error;

/// A simple serializer that can dump out strings.
pub struct PlainSerializer;

macro_rules! serialize_as_string {
    ($($ty:ty => $meth:ident,)*) => {
        $(fn $meth(self, v: $ty) -> Result<String, Error> { Ok(v.to_string()) })*
    };
}

impl Serializer for PlainSerializer {
    type Ok = String;
    type Error = Error;
    type SerializeSeq = Impossible<String, Error>;
    type SerializeTuple = Impossible<String, Error>;
    type SerializeTupleStruct = Impossible<String, Error>;
    type SerializeTupleVariant = Impossible<String, Error>;
    type SerializeMap = Impossible<String, Error>;
    type SerializeStruct = Impossible<String, Error>;
    type SerializeStructVariant = Impossible<String, Error>;

    serialize_as_string!{
        bool => serialize_bool,
        u8  => serialize_u8,
        u16 => serialize_u16,
        u32 => serialize_u32,
        u64 => serialize_u64,
        i8  => serialize_i8,
        i16 => serialize_i16,
        i32 => serialize_i32,
        i64 => serialize_i64,
        f32 => serialize_f32,
        f64 => serialize_f64,
        char => serialize_char,
        &str => serialize_str,
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<String, Error> {
        Err(Error::ImpossibleSerialization)
    }

    fn serialize_unit(self) -> Result<String, Error> {
        Ok("".to_string())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<String, Error> {
        Err(Error::ImpossibleSerialization)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<String, Error> {
        Ok(variant.to_string())
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<String, Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<String, Error> {
        Err(Error::ImpossibleSerialization)
    }

    fn serialize_none(self) -> Result<String, Error> {
        Ok("".to_string())
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<String, Error> {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        Err(Error::ImpossibleSerialization)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Error> {
        Err(Error::ImpossibleSerialization)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        Err(Error::ImpossibleSerialization)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        Err(Error::ImpossibleSerialization)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        Err(Error::ImpossibleSerialization)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        Err(Error::ImpossibleSerialization)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        Err(Error::ImpossibleSerialization)
    }
}

/// Takes a serializable object and dumps out a plain string.
///
/// This serializes an object with the `PlainSerializer` into a string and then
/// returns it.  This requires that the type is a simple one (integer, string or
/// an enum that is serialized into a string)
pub fn to_string<T: Serialize>(value: &T) -> Result<String, Error> {
    value.serialize(PlainSerializer)
}
