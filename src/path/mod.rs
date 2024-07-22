use std::str::FromStr;

use crate::error::{ConfigError, Result};
use crate::map::Map;
use crate::value::{Value, ValueKind};

mod parser;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Expression {
    Identifier(String),
    Child(Box<Self>, String),
    Subscript(Box<Self>, isize),
}

impl FromStr for Expression {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self> {
        parser::from_str(s).map_err(|err| ConfigError::PathParse(err.to_string()))
    }
}

fn sindex_to_uindex(index: isize, len: usize) -> usize {
    if index >= 0 {
        index as usize
    } else {
        len - index.unsigned_abs()
    }
}

impl Expression {
    pub fn get(self, root: &Value) -> Option<&Value> {
        match self {
            Self::Identifier(id) => {
                match root.kind {
                    // `x` access on a table is equivalent to: map[x]
                    ValueKind::Table(ref map) => map.get(&id),

                    // all other variants return None
                    _ => None,
                }
            }

            Self::Child(expr, key) => {
                match expr.get(root) {
                    Some(value) => {
                        match value.kind {
                            // Access on a table is identical to Identifier, it just forwards
                            ValueKind::Table(ref map) => map.get(&key),

                            // all other variants return None
                            _ => None,
                        }
                    }

                    _ => None,
                }
            }

            Self::Subscript(expr, index) => match expr.get(root) {
                Some(value) => match value.kind {
                    ValueKind::Array(ref array) => {
                        let index = sindex_to_uindex(index, array.len());

                        if index >= array.len() {
                            None
                        } else {
                            Some(&array[index])
                        }
                    }

                    _ => None,
                },

                _ => None,
            },
        }
    }

    pub fn get_mut<'a>(&self, root: &'a mut Value) -> Option<&'a mut Value> {
        match *self {
            Self::Identifier(ref id) => match root.kind {
                ValueKind::Table(ref mut map) => map.get_mut(id),

                _ => None,
            },

            Self::Child(ref expr, ref key) => match expr.get_mut(root) {
                Some(value) => match value.kind {
                    ValueKind::Table(ref mut map) => map.get_mut(key),

                    _ => None,
                },

                _ => None,
            },

            Self::Subscript(ref expr, index) => match expr.get_mut(root) {
                Some(value) => match value.kind {
                    ValueKind::Array(ref mut array) => {
                        let index = sindex_to_uindex(index, array.len());

                        if index >= array.len() {
                            None
                        } else {
                            Some(&mut array[index])
                        }
                    }

                    _ => None,
                },

                _ => None,
            },
        }
    }

    pub fn get_mut_forcibly<'a>(&self, root: &'a mut Value) -> Option<&'a mut Value> {
        match *self {
            Self::Identifier(ref id) => match root.kind {
                ValueKind::Table(ref mut map) => Some(
                    map.entry(id.to_lowercase())
                        .or_insert_with(|| Value::new(None, ValueKind::Nil)),
                ),

                _ => None,
            },

            Self::Child(ref expr, ref key) => match expr.get_mut_forcibly(root) {
                Some(value) => {
                    if let ValueKind::Table(ref mut map) = value.kind {
                        Some(
                            map.entry(key.to_lowercase())
                                .or_insert_with(|| Value::new(None, ValueKind::Nil)),
                        )
                    } else {
                        *value = Map::<String, Value>::new().into();

                        if let ValueKind::Table(ref mut map) = value.kind {
                            Some(
                                map.entry(key.to_lowercase())
                                    .or_insert_with(|| Value::new(None, ValueKind::Nil)),
                            )
                        } else {
                            unreachable!();
                        }
                    }
                }

                _ => None,
            },

            Self::Subscript(ref expr, index) => match expr.get_mut_forcibly(root) {
                Some(value) => {
                    match value.kind {
                        ValueKind::Array(_) => (),
                        _ => *value = Vec::<Value>::new().into(),
                    }

                    match value.kind {
                        ValueKind::Array(ref mut array) => {
                            let index = sindex_to_uindex(index, array.len());

                            if index >= array.len() {
                                array.resize(index + 1, Value::new(None, ValueKind::Nil));
                            }

                            Some(&mut array[index])
                        }

                        _ => None,
                    }
                }
                _ => None,
            },
        }
    }

    pub fn set(&self, root: &mut Value, value: Value) {
        match *self {
            Self::Identifier(ref id) => {
                // Ensure that root is a table
                match root.kind {
                    ValueKind::Table(_) => {}

                    _ => {
                        *root = Map::<String, Value>::new().into();
                    }
                }

                match value.kind {
                    ValueKind::Table(ref incoming_map) => {
                        // Pull out another table
                        let target = if let ValueKind::Table(ref mut map) = root.kind {
                            map.entry(id.to_lowercase())
                                .or_insert_with(|| Map::<String, Value>::new().into())
                        } else {
                            unreachable!();
                        };

                        // Continue the deep merge
                        for (key, val) in incoming_map {
                            Self::Identifier(key.to_lowercase()).set(target, val.clone());
                        }
                    }

                    _ => {
                        if let ValueKind::Table(ref mut map) = root.kind {
                            // Just do a simple set
                            if let Some(existing) = map.get_mut(&id.to_lowercase()) {
                                *existing = value;
                            } else {
                                map.insert(id.to_lowercase(), value);
                            }
                        }
                    }
                }
            }

            Self::Child(ref expr, ref key) => {
                if let Some(parent) = expr.get_mut_forcibly(root) {
                    if !matches!(parent.kind, ValueKind::Table(_)) {
                        // Didn't find a table. Oh well. Make a table and do this anyway
                        *parent = Map::<String, Value>::new().into();
                    }
                    Self::Identifier(key.to_lowercase()).set(parent, value);
                }
            }

            Self::Subscript(ref expr, index) => {
                if let Some(parent) = expr.get_mut_forcibly(root) {
                    if !matches!(parent.kind, ValueKind::Array(_)) {
                        *parent = Vec::<Value>::new().into()
                    }

                    if let ValueKind::Array(ref mut array) = parent.kind {
                        let uindex = sindex_to_uindex(index, array.len());
                        if uindex >= array.len() {
                            array.resize(uindex + 1, Value::new(None, ValueKind::Nil));
                        }

                        array[uindex] = value;
                    }
                }
            }
        }
    }
}
