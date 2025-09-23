use crate::value::{Value, ValueKind};
use crate::{format, Map};
use std::error::Error;

pub(crate) fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    let value = from_corn_value(uri, &corn::parse(text)?);
    format::extract_root_table(uri, value)
}

fn from_corn_value(uri: Option<&String>, value: &corn::Value<'_>) -> Value {
    match value {
        corn::Value::String(value) => Value::new(uri, ValueKind::String(value.to_string())),
        corn::Value::Integer(value) => Value::new(uri, ValueKind::I64(*value)),
        corn::Value::Float(value) => Value::new(uri, ValueKind::Float(*value)),
        corn::Value::Boolean(value) => Value::new(uri, ValueKind::Boolean(*value)),
        corn::Value::Object(value) => Value::new(
            uri,
            ValueKind::Table(
                value
                    .iter()
                    .map(|(key, value)| (key.to_string(), from_corn_value(uri, value)))
                    .collect(),
            ),
        ),
        corn::Value::Array(value) => Value::new(
            uri,
            ValueKind::Array(
                value
                    .iter()
                    .map(|value| from_corn_value(uri, value))
                    .collect(),
            ),
        ),
        corn::Value::Null(_) => Value::new(uri, ValueKind::Nil),
    }
}
