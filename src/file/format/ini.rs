use std::error::Error;

use ini::Ini;

use crate::map::Map;
use crate::value::{Table, Value, ValueKind};

pub(crate) fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    let value = from_ini(uri, Ini::load_from_str(text)?);

    match value.kind {
        ValueKind::Table(map) => Ok(map),

        _ => Ok(Map::new()),
    }
}

fn from_ini(
    uri: Option<&String>,
    data: Ini,
) -> Value {
    let mut map = Map::<String, Value>::new();

    let mut sections: Map<Option<&str>, Table> = data.into_iter().map(|(section, props)| {
        let key = section;
        let value = props.iter().map(|(k, v)| {
            let key = k.to_owned();
            let value = Value::new(uri, ValueKind::String(v.to_owned()));
            (key, value)
        }).collect();
        (key, value)
    }).collect();

    // Hoist (optional) sectionless properties to the top-level, alongside sections:
    map.extend(sections.remove(&None).unwrap_or_default());

    // Wrap each section Table into Value for merging into `map`:
    map.extend(sections.into_iter().map(|(k,v)| {
        let key = k.unwrap_or_default().to_owned();
        let value = Value::new(uri, ValueKind::Table(v));
        (key , value)
    }));

    Value::new(uri, ValueKind::Table(map))
}
