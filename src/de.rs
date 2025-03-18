use std::collections::VecDeque;
use std::convert::TryInto;
use std::iter::Enumerate;

use serde::de;

use crate::config::Config;
use crate::error::{ConfigError, Result, Unexpected};
use crate::map::Map;
use crate::value::{Table, Value, ValueKind};

macro_rules! try_convert_number {
    (signed, $self:expr, $size:literal) => {{
        let num = $self.into_int()?;
        num.try_into().map_err(|_| {
            ConfigError::invalid_type(
                None,
                Unexpected::I64(num),
                concat!("an signed ", $size, " bit integer"),
            )
        })?
    }};

    (unsigned, $self:expr, $size:literal) => {{
        let num = $self.into_uint()?;
        num.try_into().map_err(|_| {
            ConfigError::invalid_type(
                None,
                Unexpected::U64(num),
                concat!("an unsigned ", $size, " bit integer"),
            )
        })?
    }};
}

impl<'de> de::Deserializer<'de> for Value {
    type Error = ConfigError;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        // Deserialize based on the underlying type
        match self.kind {
            ValueKind::Nil => visitor.visit_unit(),
            ValueKind::I64(i) => visitor.visit_i64(i),
            ValueKind::I128(i) => visitor.visit_i128(i),
            ValueKind::U64(i) => visitor.visit_u64(i),
            ValueKind::U128(i) => visitor.visit_u128(i),
            ValueKind::Boolean(b) => visitor.visit_bool(b),
            ValueKind::Float(f) => visitor.visit_f64(f),
            ValueKind::String(s) => visitor.visit_string(s),
            ValueKind::Array(values) => visitor.visit_seq(SeqAccess::new(values)),
            ValueKind::Table(map) => visitor.visit_map(MapAccess::new(map)),
        }
    }

    #[inline]
    fn deserialize_bool<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_bool(self.into_bool()?)
    }

    #[inline]
    fn deserialize_i8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let num = try_convert_number!(signed, self, "8");
        visitor.visit_i8(num)
    }

    #[inline]
    fn deserialize_i16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let num = try_convert_number!(signed, self, "16");
        visitor.visit_i16(num)
    }

    #[inline]
    fn deserialize_i32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let num = try_convert_number!(signed, self, "32");
        visitor.visit_i32(num)
    }

    #[inline]
    fn deserialize_i64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let num = try_convert_number!(signed, self, "64");
        visitor.visit_i64(num)
    }

    #[inline]
    fn deserialize_u8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let num = try_convert_number!(unsigned, self, "8");
        visitor.visit_u8(num)
    }

    #[inline]
    fn deserialize_u16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let num = try_convert_number!(unsigned, self, "16");
        visitor.visit_u16(num)
    }

    #[inline]
    fn deserialize_u32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let num = try_convert_number!(unsigned, self, "32");
        visitor.visit_u32(num)
    }

    #[inline]
    fn deserialize_u64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let num = try_convert_number!(unsigned, self, "u64");
        visitor.visit_u64(num)
    }

    #[inline]
    fn deserialize_f32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f32(self.into_float()? as f32)
    }

    #[inline]
    fn deserialize_f64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f64(self.into_float()?)
    }

    #[inline]
    fn deserialize_str<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_string(self.into_string()?)
    }

    #[inline]
    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_string(self.into_string()?)
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        // Match an explicit nil as None and everything else as Some
        match self.kind {
            ValueKind::Nil => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(EnumAccess {
            value: self,
            name,
            variants,
        })
    }

    serde::forward_to_deserialize_any! {
        char seq
        bytes byte_buf map struct unit
        identifier ignored_any unit_struct tuple_struct tuple
    }
}

struct StrDeserializer<'a>(&'a str);

impl<'de> de::Deserializer<'de> for StrDeserializer<'_> {
    type Error = ConfigError;

    #[inline]
    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_str(self.0)
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map struct unit enum newtype_struct
        identifier ignored_any unit_struct tuple_struct tuple option
    }
}

struct SeqAccess {
    elements: Enumerate<::std::vec::IntoIter<Value>>,
}

impl SeqAccess {
    fn new(elements: Vec<Value>) -> Self {
        Self {
            elements: elements.into_iter().enumerate(),
        }
    }
}

impl<'de> de::SeqAccess<'de> for SeqAccess {
    type Error = ConfigError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.elements.next() {
            Some((idx, value)) => seed
                .deserialize(value)
                .map(Some)
                .map_err(|e| e.prepend_index(idx)),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.elements.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}

struct MapAccess {
    elements: VecDeque<(String, Value)>,
}

impl MapAccess {
    fn new(table: Map<String, Value>) -> Self {
        Self {
            elements: table.into_iter().collect(),
        }
    }
}

impl<'de> de::MapAccess<'de> for MapAccess {
    type Error = ConfigError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if let Some((ref key_s, _)) = self.elements.front() {
            let key_de = Value::new(None, key_s as &str);
            let key = de::DeserializeSeed::deserialize(seed, key_de)?;

            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        let (key, value) = self.elements.pop_front().unwrap();
        de::DeserializeSeed::deserialize(seed, value).map_err(|e| e.prepend_key(&key))
    }
}

struct EnumAccess {
    value: Value,
    name: &'static str,
    variants: &'static [&'static str],
}

impl EnumAccess {
    fn variant_deserializer(&self, name: &str) -> Result<StrDeserializer<'_>> {
        self.variants
            .iter()
            .find(|&&s| s == name)
            .map(|&s| StrDeserializer(s))
            .ok_or_else(|| self.no_constructor_error(name))
    }

    fn table_deserializer(&self, table: &Table) -> Result<StrDeserializer<'_>> {
        if table.len() == 1 {
            self.variant_deserializer(table.iter().next().unwrap().0)
        } else {
            Err(self.structural_error())
        }
    }

    fn no_constructor_error(&self, supposed_variant: &str) -> ConfigError {
        ConfigError::Message(format!(
            "enum {} does not have variant constructor {}",
            self.name, supposed_variant
        ))
    }

    fn structural_error(&self) -> ConfigError {
        ConfigError::Message(format!(
            "value of enum {} should be represented by either string or table with exactly one key",
            self.name
        ))
    }
}

impl<'de> de::EnumAccess<'de> for EnumAccess {
    type Error = ConfigError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let value = {
            let deserializer = match self.value.kind {
                ValueKind::String(ref s) => self.variant_deserializer(s),
                ValueKind::Table(ref t) => self.table_deserializer(t),
                _ => Err(self.structural_error()),
            }?;
            seed.deserialize(deserializer)?
        };

        Ok((value, self))
    }
}

impl<'de> de::VariantAccess<'de> for EnumAccess {
    type Error = ConfigError;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.value.kind {
            ValueKind::Table(t) => seed.deserialize(t.into_iter().next().unwrap().1),
            _ => unreachable!(),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value.kind {
            ValueKind::Table(t) => {
                de::Deserializer::deserialize_seq(t.into_iter().next().unwrap().1, visitor)
            }
            _ => unreachable!(),
        }
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value.kind {
            ValueKind::Table(t) => {
                de::Deserializer::deserialize_map(t.into_iter().next().unwrap().1, visitor)
            }
            _ => unreachable!(),
        }
    }
}

/// Define `$method`s, `deserialize_foo`, by forwarding to `Value`
///
/// `($arg: $argtype, ...)`, if supplied, are the formal arguments
macro_rules! config_deserialize_via_value { { $(
    $method:ident $( ( $( $arg:ident: $argtype:ty ),* ) )? ;
)* } => { $(
    #[inline]
        fn $method<V: de::Visitor<'de>>(
            self,
      $( $( $arg: $argtype, )* )?
            visitor: V,
        ) -> Result<V::Value> {
        self.cache.$method( $( $( $arg, )* )? visitor)
    }
)* } }

impl<'de> de::Deserializer<'de> for Config {
    type Error = ConfigError;

    config_deserialize_via_value! {
        deserialize_any;
        deserialize_bool;
        deserialize_i8;
        deserialize_i16;
        deserialize_i32;
        deserialize_i64;
        deserialize_u8;
        deserialize_u16;
        deserialize_u32;
        deserialize_u64;
        deserialize_f32;
        deserialize_f64;
        deserialize_str;
        deserialize_string;
        deserialize_option;

        deserialize_char;
        deserialize_seq;
        deserialize_bytes;
        deserialize_byte_buf;
        deserialize_map;
        deserialize_unit;
        deserialize_identifier;
        deserialize_ignored_any;

        deserialize_enum(name: &'static str, variants: &'static [&'static str]);
        deserialize_unit_struct(name: &'static str);
        deserialize_newtype_struct(name: &'static str);
        deserialize_tuple(n: usize);
        deserialize_tuple_struct(name: &'static str, n: usize);
        deserialize_struct(name: &'static str, fields: &'static [&'static str]);
    }
}
