use std::error::Error;
use std::io::Cursor;

use crate::map::Map;
use crate::value::{Value, ValueKind};

pub(crate) fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    let mut map: Map<String, Value> = Map::new();
    let str_iter = dotenvy::Iter::new(Cursor::new(text));
    for item in str_iter {
        let (key, value) = item?;
        map.insert(key, Value::new(uri, ValueKind::String(value)));
    }

    Ok(map)
}
