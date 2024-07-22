use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Peekable;
use std::num::ParseIntError;
use std::str::Chars;

use crate::path::Expression;

#[derive(Debug)]
pub(crate) enum ParseError {
    ExpectedIdentifier,
    ExpectedIndex,
    ExpectedIndexEnd,
    InvalidInteger(String, ParseIntError),
    UnexpectedChar(char),
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::ExpectedIdentifier => "Expected an identifier".to_string(),
            Self::ExpectedIndex => "Expected an index".to_string(),
            Self::ExpectedIndexEnd => "Expected an index to end with ']'".to_string(),
            Self::InvalidInteger(text, parse_int_error) => {
                format!("Invalid integer \"{text}\": {parse_int_error}")
            }
            Self::UnexpectedChar(c) => format!("Unexpected character '{c}'"),
        };
        f.write_str(&msg)
    }
}

fn next_identifier(chars: &mut Peekable<Chars>) -> Result<Option<String>, ParseError> {
    fn is_identifier_char(c: char) -> bool {
        c == '-' || c == '_' || c.is_alphabetic() || c.is_ascii_digit()
    }

    let mut ident = String::with_capacity(16);

    while let Some(&c) = chars.peek() {
        if is_identifier_char(c) {
            ident.push(chars.next().unwrap());
        } else {
            break;
        }
    }

    Ok((!ident.is_empty()).then_some(ident))
}

fn next_integer(chars: &mut Peekable<Chars>) -> Result<Option<isize>, ParseError> {
    let mut ident = String::with_capacity(8);

    // May start with a hyphen.
    match chars.next() {
        Some(c) if c == '-' || c.is_ascii_digit() => ident.push(c),
        Some(c) => return Err(ParseError::UnexpectedChar(c)),
        None => return Err(ParseError::ExpectedIndex),
    }

    // Rest of the index.
    while let Some(&c) = chars.peek() {
        if c == ']' {
            break;
        } else if c.is_ascii_digit() {
            ident.push(chars.next().unwrap());
        } else {
            return Err(ParseError::UnexpectedChar(c));
        }
    }

    // Must end with a bracket.
    if chars.next() != Some(']') {
        return Err(ParseError::ExpectedIndexEnd);
    }

    if ident.is_empty() {
        Ok(None)
    } else {
        Ok(Some(
            ident
                .parse()
                .map_err(|err| ParseError::InvalidInteger(ident, err))?,
        ))
    }
}

pub(crate) fn from_str(input: &str) -> Result<Expression, ParseError> {
    let mut chars = input.chars().peekable();

    // Must start with an identifier.
    let ident = next_identifier(&mut chars)?;
    let mut expr = match ident {
        Some(ident) => Expression::Identifier(ident),
        None => return Err(ParseError::ExpectedIdentifier),
    };

    while let Some(c) = chars.next() {
        expr = match c {
            '.' => {
                let ident = next_identifier(&mut chars)?;
                match ident {
                    Some(ident) => Expression::Child(Box::new(expr.clone()), ident),
                    None => return Err(ParseError::ExpectedIdentifier),
                }
            }
            '[' => match next_integer(&mut chars)? {
                Some(index) => Expression::Subscript(Box::new(expr), index),
                None => return Err(ParseError::ExpectedIndex),
            },
            c => return Err(ParseError::UnexpectedChar(c)),
        }
    }
    Ok(expr)
}

#[cfg(test)]
mod test {
    use super::Expression::*;
    use super::*;

    #[test]
    fn test_identifier() {
        let parsed: Expression = from_str("abcd").unwrap();
        assert_eq!(parsed, Identifier("abcd".to_string()));

        // Unicode is permitted in identifiers. Identifiers must still be alphabetic.
        let parsed: Expression = from_str("Öyster").unwrap();
        assert_eq!(parsed, Identifier("Öyster".to_string()));
    }

    #[test]
    fn test_identifier_chars() {
        let parsed: Expression = from_str("abcd-efgh").unwrap();
        assert_eq!(parsed, Identifier("abcd-efgh".to_string()));

        let parsed: Expression = from_str("abcd_efgh").unwrap();
        assert_eq!(parsed, Identifier("abcd_efgh".to_string()));
    }

    #[test]
    fn test_child() {
        let parsed: Expression = from_str("abcd.efgh").unwrap();
        let expected = Child(Box::new(Identifier("abcd".to_string())), "efgh".to_string());
        assert_eq!(parsed, expected);

        let parsed: Expression = from_str("abcd.efgh.ijkl").unwrap();
        let expected = Child(
            Box::new(Child(
                Box::new(Identifier("abcd".to_string())),
                "efgh".to_string(),
            )),
            "ijkl".to_string(),
        );
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_subscript() {
        let parsed: Expression = from_str("abcd[12]").unwrap();
        let expected = Subscript(Box::new(Identifier("abcd".to_string())), 12);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_subscript_negative() {
        let parsed: Expression = from_str("abcd[-1]").unwrap();
        let expected = Subscript(Box::new(Identifier("abcd".to_string())), -1);
        assert_eq!(parsed, expected);
    }
}
