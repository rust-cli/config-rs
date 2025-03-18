use std::error::Error;

use crate::format;
use crate::map::Map;
use crate::value::{Value, ValueKind};

pub(crate) fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    // Parse a JSON object value from the text
    let value = from_json_value(uri, &serde_json::from_str(text)?);
    format::extract_root_table(uri, value)
}

fn from_json_value(uri: Option<&String>, value: &serde_json::Value) -> Value {
    match *value {
        serde_json::Value::String(ref value) => Value::new(uri, ValueKind::String(value.clone())),

        serde_json::Value::Number(ref value) => {
            if let Some(value) = value.as_i64() {
                Value::new(uri, ValueKind::I64(value))
            } else if let Some(value) = value.as_f64() {
                Value::new(uri, ValueKind::Float(value))
            } else {
                unreachable!();
            }
        }

        serde_json::Value::Bool(value) => Value::new(uri, ValueKind::Boolean(value)),

        serde_json::Value::Object(ref table) => {
            let mut m = Map::new();

            for (key, value) in table {
                m.insert(key.clone(), from_json_value(uri, value));
            }

            Value::new(uri, ValueKind::Table(m))
        }

        serde_json::Value::Array(ref array) => {
            let mut l = Vec::new();

            for value in array {
                l.push(from_json_value(uri, value));
            }

            Value::new(uri, ValueKind::Array(l))
        }

        serde_json::Value::Null => Value::new(uri, ValueKind::Nil),
    }
}
