use std::str::FromStr;

use crate::error::{ConfigError, Result};
use crate::map::Map;
use crate::value::{Value, ValueKind};

mod parser;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub(crate) enum Expression {
    Identifier(String),
    Child(Box<Self>, String),
    Subscript(Box<Self>, isize),
}

impl Expression {
    pub(crate) fn root(root: String) -> Self {
        Expression::Identifier(root)
    }
}

impl FromStr for Expression {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self> {
        parser::from_str(s).map_err(|e| ConfigError::PathParse {
            cause: Box::new(ParseError::new(e)),
        })
    }
}

#[derive(Debug)]
struct ParseError(String);

impl ParseError {
    fn new(inner: winnow::error::ParseError<&str, winnow::error::ContextError>) -> Self {
        Self(inner.to_string())
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for ParseError {}

/// Convert a relative index into an absolute index
fn abs_index(index: isize, len: usize) -> Result<usize, usize> {
    if index >= 0 {
        Ok(index as usize)
    } else if let Some(index) = len.checked_sub(index.unsigned_abs()) {
        Ok(index)
    } else {
        Err((len as isize + index).unsigned_abs())
    }
}

impl Expression {
    pub(crate) fn get(self, root: &Value) -> Option<&Value> {
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
                        let index = abs_index(index, array.len()).ok()?;
                        array.get(index)
                    }

                    _ => None,
                },

                _ => None,
            },
        }
    }

    pub(crate) fn get_mut_forcibly<'a>(&self, root: &'a mut Value) -> Option<&'a mut Value> {
        match *self {
            Self::Identifier(ref id) => match root.kind {
                ValueKind::Table(ref mut map) => Some(
                    map.entry(id.clone())
                        .or_insert_with(|| Value::new(None, ValueKind::Nil)),
                ),

                _ => None,
            },

            Self::Child(ref expr, ref key) => match expr.get_mut_forcibly(root) {
                Some(value) => {
                    if let ValueKind::Table(ref mut map) = value.kind {
                        Some(
                            map.entry(key.clone())
                                .or_insert_with(|| Value::new(None, ValueKind::Nil)),
                        )
                    } else {
                        *value = Map::<String, Value>::new().into();

                        if let ValueKind::Table(ref mut map) = value.kind {
                            Some(
                                map.entry(key.clone())
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
                            let uindex = match abs_index(index, array.len()) {
                                Ok(uindex) => {
                                    if uindex >= array.len() {
                                        array.resize(uindex + 1, Value::new(None, ValueKind::Nil));
                                    }
                                    uindex
                                }
                                Err(insertion) => {
                                    array.splice(
                                        0..0,
                                        (0..insertion).map(|_| Value::new(None, ValueKind::Nil)),
                                    );
                                    0
                                }
                            };

                            Some(&mut array[uindex])
                        }

                        _ => unreachable!(),
                    }
                }
                _ => None,
            },
        }
    }

    pub(crate) fn set(&self, root: &mut Value, value: Value) {
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
                            map.entry(id.clone())
                                .or_insert_with(|| Map::<String, Value>::new().into())
                        } else {
                            unreachable!();
                        };

                        // Continue the deep merge
                        for (key, val) in incoming_map {
                            Self::Identifier(key.clone()).set(target, val.clone());
                        }
                    }

                    _ => {
                        if let ValueKind::Table(ref mut map) = root.kind {
                            // Just do a simple set
                            if let Some(existing) = map.get_mut(id) {
                                *existing = value;
                            } else {
                                map.insert(id.clone(), value);
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
                    Self::Identifier(key.clone()).set(parent, value);
                }
            }

            Self::Subscript(ref expr, index) => {
                if let Some(parent) = expr.get_mut_forcibly(root) {
                    if !matches!(parent.kind, ValueKind::Array(_)) {
                        *parent = Vec::<Value>::new().into();
                    }

                    if let ValueKind::Array(ref mut array) = parent.kind {
                        let uindex = match abs_index(index, array.len()) {
                            Ok(uindex) => {
                                if uindex >= array.len() {
                                    array.resize(uindex + 1, Value::new(None, ValueKind::Nil));
                                }
                                uindex
                            }
                            Err(insertion) => {
                                array.splice(
                                    0..0,
                                    (0..insertion).map(|_| Value::new(None, ValueKind::Nil)),
                                );
                                0
                            }
                        };

                        array[uindex] = value;
                    }
                }
            }
        }
    }
}
