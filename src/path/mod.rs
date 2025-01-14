use std::str::FromStr;

use crate::error::{ConfigError, Result};
use crate::map::Map;
use crate::value::{Value, ValueKind};

mod parser;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub(crate) struct Expression {
    root: String,
    postfix: Vec<Postfix>,
}

impl Expression {
    pub(crate) fn root(root: String) -> Self {
        Self {
            root,
            postfix: Vec::new(),
        }
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

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Postfix {
    Key(String),
    Index(isize),
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
        let ValueKind::Table(map) = &root.kind else {
            return None;
        };
        let mut child = map.get(&self.root)?;
        for postfix in &self.postfix {
            match postfix {
                Postfix::Key(key) => {
                    let ValueKind::Table(map) = &child.kind else {
                        return None;
                    };
                    child = map.get(key)?;
                }
                Postfix::Index(rel_index) => {
                    let ValueKind::Array(array) = &child.kind else {
                        return None;
                    };
                    let index = abs_index(*rel_index, array.len()).ok()?;
                    child = array.get(index)?;
                }
            }
        }
        Some(child)
    }

    pub(crate) fn get_mut_forcibly<'a>(&self, root: &'a mut Value) -> &'a mut Value {
        if !matches!(root.kind, ValueKind::Table(_)) {
            *root = Map::<String, Value>::new().into();
        }
        let ValueKind::Table(map) = &mut root.kind else {
            unreachable!()
        };
        let mut child = map
            .entry(self.root.clone())
            .or_insert_with(|| Value::new(None, ValueKind::Nil));
        for postfix in &self.postfix {
            match postfix {
                Postfix::Key(key) => {
                    if !matches!(child.kind, ValueKind::Table(_)) {
                        *child = Map::<String, Value>::new().into();
                    }
                    let ValueKind::Table(ref mut map) = child.kind else {
                        unreachable!()
                    };

                    child = map
                        .entry(key.clone())
                        .or_insert_with(|| Value::new(None, ValueKind::Nil));
                }
                Postfix::Index(rel_index) => {
                    if !matches!(child.kind, ValueKind::Array(_)) {
                        *child = Vec::<Value>::new().into();
                    }
                    let ValueKind::Array(ref mut array) = child.kind else {
                        unreachable!()
                    };

                    let uindex = match abs_index(*rel_index, array.len()) {
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

                    child = &mut array[uindex];
                }
            }
        }
        child
    }

    pub(crate) fn set(&self, root: &mut Value, value: Value) {
        let parent = self.get_mut_forcibly(root);
        match value.kind {
            ValueKind::Table(ref incoming_map) => {
                // If the parent is not a table, overwrite it, treating it as a
                // table
                if !matches!(parent.kind, ValueKind::Table(_)) {
                    *parent = Map::<String, Value>::new().into();
                }

                // Continue the deep merge
                for (key, val) in incoming_map {
                    Self::root(key.clone()).set(parent, val.clone());
                }
            }
            _ => {
                *parent = value;
            }
        }
    }
}
