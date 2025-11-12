use std::error::Error;

use crate::format;
use crate::map::Map;
use crate::value::{Value, ValueKind};

pub(crate) fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    let value = from_ron_value(uri, ron::from_str(text)?)?;
    format::extract_root_table(uri, value)
}

fn from_ron_value(
    uri: Option<&String>,
    value: ron::Value,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let kind = match value {
        ron::Value::Option(value) => match value {
            Some(value) => from_ron_value(uri, *value)?.kind,
            None => ValueKind::Nil,
        },

        ron::Value::Unit => ValueKind::Nil,

        ron::Value::Bytes(value) => ValueKind::String(String::from_utf8_lossy(&value).into_owned()),

        ron::Value::Bool(value) => ValueKind::Boolean(value),

        ron::Value::Number(value) => {
            match value {
                ron::Number::I8(v) => ValueKind::I64(v as i64),
                ron::Number::I16(v) => ValueKind::I64(v as i64),
                ron::Number::I32(v) => ValueKind::I64(v as i64),
                ron::Number::I64(v) => ValueKind::I64(v),
                #[cfg(feature = "integer128")]
                ron::Number::I128(v) => ValueKind::I64(v as i64),
                ron::Number::U8(v) => ValueKind::I64(v as i64),
                ron::Number::U16(v) => ValueKind::I64(v as i64),
                ron::Number::U32(v) => ValueKind::I64(v as i64),
                ron::Number::U64(v) => ValueKind::I64(v as i64),
                #[cfg(feature = "integer128")]
                ron::Number::U128(v) => ValueKind::I64(v as i64),
                ron::Number::F32(v) => ValueKind::Float(v.get() as f64),
                ron::Number::F64(v) => ValueKind::Float(v.get()),
                #[cfg(not(doc))]
                ron::Number::__NonExhaustive(never) => never.never(),
            }
        },

        ron::Value::Char(value) => ValueKind::String(value.to_string()),

        ron::Value::String(value) => ValueKind::String(value),

        ron::Value::Seq(values) => {
            let array = values
                .into_iter()
                .map(|value| from_ron_value(uri, value))
                .collect::<Result<Vec<_>, _>>()?;

            ValueKind::Array(array)
        }

        ron::Value::Map(values) => {
            let map = values
                .iter()
                .map(|(key, value)| -> Result<_, Box<dyn Error + Send + Sync>> {
                    let key = key.clone().into_rust::<String>()?;
                    let value = from_ron_value(uri, value.clone())?;

                    Ok((key, value))
                })
                .collect::<Result<Map<_, _>, _>>()?;

            ValueKind::Table(map)
        }
    };

    Ok(Value::new(uri, kind))
}
