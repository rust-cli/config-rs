use std::error::Error;

use ini::Ini;

use crate::map::Map;
use crate::value::{Value, ValueKind};

pub(crate) fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    let mut map: Map<String, Value> = Map::new();
    let i = Ini::load_from_str(text)?;
    for (sec, prop) in i.iter() {
        match sec {
            Some(sec) => {
                let mut sec_map: Map<String, Value> = Map::new();
                for (k, v) in prop.iter() {
                    sec_map.insert(
                        k.to_owned(),
                        Value::new(uri, ValueKind::String(v.to_owned())),
                    );
                }
                map.insert(sec.to_owned(), Value::new(uri, ValueKind::Table(sec_map)));
            }
            None => {
                for (k, v) in prop.iter() {
                    map.insert(
                        k.to_owned(),
                        Value::new(uri, ValueKind::String(v.to_owned())),
                    );
                }
            }
        }
    }
    Ok(map)
}
