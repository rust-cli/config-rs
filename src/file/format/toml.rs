use std::error::Error;

use crate::map::Map;
use crate::value::Value;

pub(crate) fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    // Parse a TOML value from the provided text
    let table = from_toml_table(uri, toml::from_str(text)?);
    Ok(table)
}

fn from_toml_table(uri: Option<&String>, table: toml::Table) -> Map<String, Value> {
    let mut m = Map::new();

    for (key, value) in table {
        m.insert(key, from_toml_value(uri, value));
    }

    m
}

fn from_toml_value(uri: Option<&String>, value: toml::Value) -> Value {
    match value {
        toml::Value::String(value) => Value::new(uri, value),
        toml::Value::Float(value) => Value::new(uri, value),
        toml::Value::Integer(value) => Value::new(uri, value),
        toml::Value::Boolean(value) => Value::new(uri, value),

        toml::Value::Table(table) => {
            let m = from_toml_table(uri, table);
            Value::new(uri, m)
        }

        toml::Value::Array(array) => {
            let mut l = Vec::new();

            for value in array {
                l.push(from_toml_value(uri, value));
            }

            Value::new(uri, l)
        }

        toml::Value::Datetime(datetime) => Value::new(uri, datetime.to_string()),
    }
}
