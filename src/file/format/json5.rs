use std::error::Error;

use crate::format;
use crate::map::Map;
use crate::value::{Value, ValueKind};

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum Val {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Self>),
    Object(Map<String, Self>),
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
