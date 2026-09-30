use std::error::Error;

use crate::map::Map;
use crate::value::Value;

pub(crate) fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    // Parse a TOML value from the provided text
    let table = from_toml_table(uri, toml::de::DeTable::parse(text)?.into_inner())?;
    Ok(table)
}

fn from_toml_table(
    uri: Option<&String>,
    table: toml::de::DeTable<'_>,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    let mut m = Map::new();

    for (key, value) in table {
        m.insert(
            key.into_inner().into_owned(),
            from_toml_value(uri, value.into_inner())?,
        );
    }

    Ok(m)
}

fn from_toml_value(
    uri: Option<&String>,
    value: toml::de::DeValue<'_>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = match value {
        toml::de::DeValue::String(value) => Value::new(uri, value.into_owned()),
        toml::de::DeValue::Float(value) => {
            let float = value.as_str().parse::<f64>()?;
            if float.is_infinite() && !value.as_str().contains("inf") {
                return Err("floating-point number overflowed".into());
            }
            Value::new(uri, float)
        }
        toml::de::DeValue::Integer(value) => {
            Value::new(uri, i64::from_str_radix(value.as_str(), value.radix())?)
        }
        toml::de::DeValue::Boolean(value) => Value::new(uri, value),

        toml::de::DeValue::Table(table) => {
            let m = from_toml_table(uri, table)?;
            Value::new(uri, m)
        }

        toml::de::DeValue::Array(array) => {
            let mut l = Vec::new();

            for value in array {
                l.push(from_toml_value(uri, value.into_inner())?);
            }

            Value::new(uri, l)
        }

        toml::de::DeValue::Datetime(datetime) => Value::new(uri, datetime.to_string()),
    };
    Ok(value)
}
