use std::fmt::Display;
use std::fmt::Write as _;

use serde::ser;

use crate::error::{ConfigError, Result};
use crate::value::{Value, ValueKind};
use crate::Config;

#[derive(Default, Debug)]
pub(crate) struct ConfigSerializer {
    keys: Vec<SerKey>,
    pub output: Config,
}

#[derive(Debug)]
enum SerKey {
    Named(String),
    Seq(usize),
}

/// An uninhabited type: no values like this can ever exist!
pub(crate) enum Unreachable {}

/// Serializer for numbered sequences
///
/// This wrapper is present when we are outputting a sequence (numbered indices).
/// Making this a separate type centralises the handling of sequences
/// and ensures we don't have any call sites for `ser::SerializeSeq::serialize_element`
/// that don't do the necessary work of `SeqSerializer::new`.
///
/// Existence of this wrapper implies that `.0.keys.last()` is
/// `Some(SerKey::Seq(next_index))`.
pub(crate) struct SeqSerializer<'a>(&'a mut ConfigSerializer);

impl ConfigSerializer {
    fn serialize_primitive<T>(&mut self, value: T) -> Result<()>
    where
        T: Into<Value> + Display,
    {
        // At some future point we could perhaps retain a cursor into the output `Config`,
        // rather than reifying the whole thing into a single string with `make_full_key`
        // and passing that whole path to the `set` method.
        //
        // That would be marginally more performant, but more fiddly.
        let key = self.make_full_key()?;

        self.output.set(&key, value.into())?;
        Ok(())
    }

    fn make_full_key(&self) -> Result<String> {
        let mut keys = self.keys.iter();

        let mut whole = match keys.next() {
            Some(SerKey::Named(s)) => s.clone(),
            _ => return Err(ConfigError::Message("top level is not a struct".to_owned())),
        };

        for k in keys {
            match k {
                SerKey::Named(s) => write!(whole, ".{s}"),
                SerKey::Seq(i) => write!(whole, "[{i}]"),
            }
            .expect("write! to a string failed");
        }

        Ok(whole)
    }

    fn push_key(&mut self, key: &str) {
        self.keys.push(SerKey::Named(key.to_owned()));
    }

    fn pop_key(&mut self) {
        self.keys.pop();
    }
}

impl<'a> ser::Serializer for &'a mut ConfigSerializer {
    type Ok = ();
    type Error = ConfigError;
    type SerializeSeq = SeqSerializer<'a>;
    type SerializeTuple = SeqSerializer<'a>;
    type SerializeTupleStruct = SeqSerializer<'a>;
    type SerializeTupleVariant = SeqSerializer<'a>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.serialize_primitive(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.serialize_i64(v.into())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.serialize_i64(v.into())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.serialize_i64(v.into())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.serialize_primitive(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.serialize_u64(v.into())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.serialize_u64(v.into())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.serialize_u64(v.into())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        if v > (i64::MAX as u64) {
            Err(ConfigError::Message(format!(
                "value {} is greater than the max {}",
                v,
                i64::MAX
            )))
        } else {
            self.serialize_i64(v as i64)
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.serialize_f64(v.into())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        self.serialize_primitive(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.serialize_primitive(v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.serialize_primitive(v.to_owned())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end();
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        self.serialize_primitive(Value::from(ValueKind::Nil))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize,
    {
        self.push_key(variant);
        value.serialize(&mut *self)?;
        self.pop_key();
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        SeqSerializer::new(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.push_key(variant);
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.push_key(variant);
        Ok(self)
    }
}

impl<'a> SeqSerializer<'a> {
    fn new(inner: &'a mut ConfigSerializer) -> Result<Self> {
        inner.keys.push(SerKey::Seq(0));

        Ok(SeqSerializer(inner))
    }

    fn end(self) -> &'a mut ConfigSerializer {
        // This ought to be Some(SerKey::Seq(..)) but we don't want to panic if we are buggy
        let _: Option<SerKey> = self.0.keys.pop();
        self.0
    }
}

impl ser::SerializeSeq for SeqSerializer<'_> {
    type Ok = ();
    type Error = ConfigError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut *(self.0))?;
        match self.0.keys.last_mut() {
            Some(SerKey::Seq(i)) => *i += 1,
            _ => {
                return Err(ConfigError::Message(
                    "config-rs internal error (ser._element but last not Seq!".to_owned(),
                ))
            }
        };
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.end();
        Ok(())
    }
}

impl ser::SerializeTuple for SeqSerializer<'_> {
    type Ok = ();
    type Error = ConfigError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleStruct for SeqSerializer<'_> {
    type Ok = ();
    type Error = ConfigError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleVariant for SeqSerializer<'_> {
    type Ok = ();
    type Error = ConfigError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        let inner = self.end();
        inner.pop_key();
        Ok(())
    }
}

impl ser::SerializeMap for &mut ConfigSerializer {
    type Ok = ();
    type Error = ConfigError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        let key_serializer = StringKeySerializer;
        let key = key.serialize(key_serializer)?;
        self.push_key(&key);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)?;
        self.pop_key();
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl ser::SerializeStruct for &mut ConfigSerializer {
    type Ok = ();
    type Error = ConfigError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        self.push_key(key);
        value.serialize(&mut **self)?;
        self.pop_key();
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl ser::SerializeStructVariant for &mut ConfigSerializer {
    type Ok = ();
    type Error = ConfigError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        self.push_key(key);
        value.serialize(&mut **self)?;
        self.pop_key();
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.pop_key();
        Ok(())
    }
}

pub(crate) struct StringKeySerializer;

/// Define `$emthod`, `serialize_foo`, taking `$type` and serialising it via [`Display`]
macro_rules! string_serialize_via_display { { $method:ident, $type:ty } => {
    fn $method(self, v: $type) -> Result<Self::Ok> {
        #[allow(clippy::str_to_string)]
        Ok(v.to_string())
    }
} }

impl ser::Serializer for StringKeySerializer {
    type Ok = String;
    type Error = ConfigError;
    type SerializeSeq = Unreachable;
    type SerializeTuple = Unreachable;
    type SerializeTupleStruct = Unreachable;
    type SerializeTupleVariant = Unreachable;
    type SerializeMap = Unreachable;
    type SerializeStruct = Unreachable;
    type SerializeStructVariant = Unreachable;

    string_serialize_via_display!(serialize_bool, bool);
    string_serialize_via_display!(serialize_i8, i8);
    string_serialize_via_display!(serialize_i16, i16);
    string_serialize_via_display!(serialize_i32, i32);
    string_serialize_via_display!(serialize_i64, i64);
    string_serialize_via_display!(serialize_u8, u8);
    string_serialize_via_display!(serialize_u16, u16);
    string_serialize_via_display!(serialize_u32, u32);
    string_serialize_via_display!(serialize_u64, u64);
    string_serialize_via_display!(serialize_f32, f32);
    string_serialize_via_display!(serialize_f64, f64);
    string_serialize_via_display!(serialize_char, char);
    string_serialize_via_display!(serialize_str, &str);

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        Ok(String::from_utf8_lossy(v).to_string())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(String::new())
    }

    fn serialize_unit_struct(self, _name: &str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &str,
        _variant_index: u32,
        variant: &str,
    ) -> Result<Self::Ok> {
        Ok(variant.to_owned())
    }

    fn serialize_newtype_struct<T>(self, _name: &str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &str,
        _variant_index: u32,
        _variant: &str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(ConfigError::Message(
            "seq can't serialize to string key".to_owned(),
        ))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(ConfigError::Message(
            "tuple can't serialize to string key".to_owned(),
        ))
    }

    fn serialize_tuple_struct(self, name: &str, _len: usize) -> Result<Self::SerializeTupleStruct> {
        Err(ConfigError::Message(format!(
            "tuple struct {name} can't serialize to string key"
        )))
    }

    fn serialize_tuple_variant(
        self,
        name: &str,
        _variant_index: u32,
        variant: &str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(ConfigError::Message(format!(
            "tuple variant {name}::{variant} can't serialize to string key"
        )))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(ConfigError::Message(
            "map can't serialize to string key".to_owned(),
        ))
    }

    fn serialize_struct(self, name: &str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(ConfigError::Message(format!(
            "struct {name} can't serialize to string key"
        )))
    }

    fn serialize_struct_variant(
        self,
        name: &str,
        _variant_index: u32,
        variant: &str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(ConfigError::Message(format!(
            "struct variant {name}::{variant} can't serialize to string key"
        )))
    }
}

impl ser::SerializeSeq for Unreachable {
    type Ok = String;
    type Error = ConfigError;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        match *self {}
    }

    fn end(self) -> Result<Self::Ok> {
        match self {}
    }
}

impl ser::SerializeTuple for Unreachable {
    type Ok = String;
    type Error = ConfigError;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        match *self {}
    }

    fn end(self) -> Result<Self::Ok> {
        match self {}
    }
}

impl ser::SerializeTupleStruct for Unreachable {
    type Ok = String;
    type Error = ConfigError;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        match *self {}
    }

    fn end(self) -> Result<Self::Ok> {
        match self {}
    }
}

impl ser::SerializeTupleVariant for Unreachable {
    type Ok = String;
    type Error = ConfigError;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        match *self {}
    }

    fn end(self) -> Result<Self::Ok> {
        match self {}
    }
}

impl ser::SerializeMap for Unreachable {
    type Ok = String;
    type Error = ConfigError;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        match *self {}
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        match *self {}
    }

    fn end(self) -> Result<Self::Ok> {
        match self {}
    }
}

impl ser::SerializeStruct for Unreachable {
    type Ok = String;
    type Error = ConfigError;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        match *self {}
    }

    fn end(self) -> Result<Self::Ok> {
        match self {}
    }
}

impl ser::SerializeStructVariant for Unreachable {
    type Ok = String;
    type Error = ConfigError;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        match *self {}
    }

    fn end(self) -> Result<Self::Ok> {
        match self {}
    }
}

#[cfg(test)]
mod test {
    use serde_derive::{Deserialize, Serialize};

    use super::*;

    #[test]
    fn test_struct() {
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
        struct Test {
            int: u32,
            seq: Vec<String>,
        }

        let test = Test {
            int: 1,
            seq: vec!["a".to_owned(), "b".to_owned()],
        };
        let config = Config::try_from(&test).unwrap();

        let actual: Test = config.try_deserialize().unwrap();
        assert_eq!(test, actual);
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_nest() {
        let val = serde_json::json! { {
            "top": {
                "num": 1,
                "array": [2],
                "nested": [[3,4]],
                "deep": [{
                    "yes": true,
                }],
                "mixed": [
                    { "boolish": false, },
                    42,
                    ["hi"],
                    { "inner": 66 },
                    23,
                ],
            }
        } };
        let config = Config::try_from(&val).unwrap();
        let output: serde_json::Value = config.try_deserialize().unwrap();
        assert_eq!(val, output);
    }
}
