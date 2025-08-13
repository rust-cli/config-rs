use std::error::Error;

use crate::format;
use crate::map::Map;
use crate::value::{Value, ValueKind};
use jsonc_parser::JsonValue;

pub(crate) fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    let value = match jsonc_parser::parse_to_value(text, &Default::default())? {
        Some(json_value) => from_jsonc_value(uri, json_value),
        None => Value::new(uri, ValueKind::Nil),
    };
    format::extract_root_table(uri, value)
}

fn from_jsonc_value(uri: Option<&String>, value: JsonValue<'_>) -> Value {
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
