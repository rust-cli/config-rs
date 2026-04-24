use std::error::Error;
use std::io::Cursor;

use crate::map::Map;
use crate::value::{Value, ValueKind};

#[cfg(feature = "convert-case")]
use convert_case::{Case, Casing};

pub(crate) fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    let mut map: Map<String, Value> = Map::new();
    let str_iter = dotenvy::Iter::new(Cursor::new(text));
    for item in str_iter {
        let (mut key, value) = item?;

        #[cfg(feature = "convert-case")]
        {
            key = key.to_case(Case::Snake);
        }

        map.insert(key, Value::new(uri, ValueKind::String(value)));
    }

    Ok(map)
}
