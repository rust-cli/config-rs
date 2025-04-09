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
        // Option<Boxed<ron::Value>> requires deref of boxed value
        ron::Value::Option(value) => match value {
            Some(boxed) => from_ron_value(uri, *boxed)?.kind,
            None => ValueKind::Nil,
        },

        ron::Value::Unit => ValueKind::Nil,

        ron::Value::Bool(value) => ValueKind::Boolean(value),

        ron::Value::Number(value) => match value {
            ron::Number::F32(value) => ValueKind::Float(value.get() as f64),
            ron::Number::F64(value) => ValueKind::Float(value.get()),
            ron::Number::I8(value) => ValueKind::I64(value.into()),
            ron::Number::I16(value) => ValueKind::I64(value.into()),
            ron::Number::I32(value) => ValueKind::I64(value.into()),
            ron::Number::I64(value) => ValueKind::I64(value),
            ron::Number::U8(value) => ValueKind::U64(value.into()),
            ron::Number::U16(value) => ValueKind::U64(value.into()),
            ron::Number::U32(value) => ValueKind::U64(value.into()),
            ron::Number::U64(value) => ValueKind::U64(value),
        },

        ron::Value::Bytes(_) => todo!(),

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
