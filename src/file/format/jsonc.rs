use std::error::Error;

use crate::error::{ConfigError, Unexpected};
use crate::map::Map;
use crate::value::{Value, ValueKind};

use jsonc_parser::JsonValue;

pub fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    match jsonc_parser::parse_to_value(text, &Default::default())? {
        Some(r) => match r {
            JsonValue::String(ref value) => Err(Unexpected::Str(value.to_string())),
            JsonValue::Number(value) => Err(Unexpected::Float(value.parse::<f64>().unwrap())),
            JsonValue::Boolean(value) => Err(Unexpected::Bool(value)),
            JsonValue::Object(o) => match from_jsonc_value(uri, JsonValue::Object(o)).kind {
                ValueKind::Table(map) => Ok(map),
                _ => unreachable!(),
            },
            JsonValue::Array(_) => Err(Unexpected::Seq),
            JsonValue::Null => Err(Unexpected::Unit),
        },
        None => Err(Unexpected::Unit),
    }
    .map_err(|err| ConfigError::invalid_root(uri, err))
    .map_err(|err| Box::new(err) as Box<dyn Error + Send + Sync>)
}

fn from_jsonc_value(uri: Option<&String>, value: JsonValue) -> Value {
    let vk = match value {
        JsonValue::Null => ValueKind::Nil,
        JsonValue::String(v) => ValueKind::String(v.to_string()),
        JsonValue::Number(number) => {
            if let Ok(v) = number.parse::<i64>() {
                ValueKind::I64(v)
            } else if let Ok(v) = number.parse::<f64>() {
                ValueKind::Float(v)
            } else {
                unreachable!();
            }
        }
        JsonValue::Boolean(v) => ValueKind::Boolean(v),
        JsonValue::Object(table) => {
            let m = table
                .into_iter()
                .map(|(k, v)| (k, from_jsonc_value(uri, v)))
                .collect();
            ValueKind::Table(m)
        }
        JsonValue::Array(array) => {
            let l = array
                .into_iter()
                .map(|v| from_jsonc_value(uri, v))
                .collect();
            ValueKind::Array(l)
        }
    };
    Value::new(uri, vk)
}
