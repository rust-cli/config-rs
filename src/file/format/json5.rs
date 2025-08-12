use std::error::Error;

use crate::format;
use crate::map::Map;
use crate::value::{Value, ValueKind};

#[derive(Debug)]
pub(crate) enum Val {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Self>),
    Object(Map<String, Self>),
}

impl<'de> serde::de::Deserialize<'de> for Val {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        serde_untagged::UntaggedEnumVisitor::new()
            .bool(|value| Ok(Self::Boolean(value)))
            .i64(|value| Ok(Self::Integer(value)))
            .f64(|value| Ok(Self::Float(value)))
            .string(|value| Ok(Val::String(value.to_owned())))
            .none(|| Ok(Self::Null))
            .seq(|value| value.deserialize().map(Val::Array))
            .map(|value| value.deserialize().map(Val::Object))
            .deserialize(d)
    }
}

pub(crate) fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    let value = from_json5_value(uri, json5_rs::from_str::<Val>(text)?);
    format::extract_root_table(uri, value)
}

fn from_json5_value(uri: Option<&String>, value: Val) -> Value {
    let vk = match value {
        Val::Null => ValueKind::Nil,
        Val::String(v) => ValueKind::String(v),
        Val::Integer(v) => ValueKind::I64(v),
        Val::Float(v) => ValueKind::Float(v),
        Val::Boolean(v) => ValueKind::Boolean(v),
        Val::Object(table) => {
            let m = table
                .into_iter()
                .map(|(k, v)| (k, from_json5_value(uri, v)))
                .collect();

            ValueKind::Table(m)
        }

        Val::Array(array) => {
            let l = array
                .into_iter()
                .map(|v| from_json5_value(uri, v))
                .collect();

            ValueKind::Array(l)
        }
    };

    Value::new(uri, vk)
}
