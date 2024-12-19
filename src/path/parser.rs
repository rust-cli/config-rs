use std::str::FromStr;

use winnow::ascii::digit1;
use winnow::ascii::space0;
use winnow::combinator::dispatch;
use winnow::combinator::eof;
use winnow::combinator::fail;
use winnow::combinator::opt;
use winnow::combinator::repeat;
use winnow::combinator::seq;
use winnow::error::ContextError;
use winnow::prelude::*;
use winnow::token::any;
use winnow::token::take_while;

use crate::path::Expression;

pub(crate) fn from_str(mut input: &str) -> Result<Expression, ContextError> {
    let input = &mut input;
    path(input).map_err(|e| e.into_inner().unwrap())
}

fn path(i: &mut &str) -> PResult<Expression> {
    let root = ident.parse_next(i)?;
    let expr = repeat(0.., postfix)
        .fold(
            || root.clone(),
            |prev, cur| match cur {
                Child::Key(k) => Expression::Child(Box::new(prev), k),
                Child::Index(k) => Expression::Subscript(Box::new(prev), k),
            },
        )
        .parse_next(i)?;
    eof.parse_next(i)?;
    Ok(expr)
}

fn ident(i: &mut &str) -> PResult<Expression> {
    raw_ident.map(Expression::Identifier).parse_next(i)
}

fn postfix(i: &mut &str) -> PResult<Child> {
    dispatch! {any;
        '[' => seq!(integer.map(Child::Index), _: ']').map(|(i,)| i),
        '.' => raw_ident.map(Child::Key),
        _ => fail,
    }
    .parse_next(i)
}

enum Child {
    Key(String),
    Index(isize),
}

fn raw_ident(i: &mut &str) -> PResult<String> {
    take_while(1.., ('a'..='z', 'A'..='Z', '0'..='9', '_', '-'))
        .map(ToString::to_string)
        .parse_next(i)
}

fn integer(i: &mut &str) -> PResult<isize> {
    seq!(
        _: space0,
        (opt('-'), digit1).take().try_map(FromStr::from_str),
        _: space0
    )
    .map(|(i,)| i)
    .parse_next(i)
}

#[cfg(test)]
mod test {
    use super::Expression::*;
    use super::*;

    #[test]
    fn test_id() {
        let parsed: Expression = from_str("abcd").unwrap();
        assert_eq!(parsed, Identifier("abcd".into()));
    }

    #[test]
    fn test_id_dash() {
        let parsed: Expression = from_str("abcd-efgh").unwrap();
        assert_eq!(parsed, Identifier("abcd-efgh".into()));
    }

    #[test]
    fn test_child() {
        let parsed: Expression = from_str("abcd.efgh").unwrap();
        let expected = Child(Box::new(Identifier("abcd".into())), "efgh".into());

        assert_eq!(parsed, expected);

        let parsed: Expression = from_str("abcd.efgh.ijkl").unwrap();
        let expected = Child(
            Box::new(Child(Box::new(Identifier("abcd".into())), "efgh".into())),
            "ijkl".into(),
        );

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_subscript() {
        let parsed: Expression = from_str("abcd[12]").unwrap();
        let expected = Subscript(Box::new(Identifier("abcd".into())), 12);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_subscript_neg() {
        let parsed: Expression = from_str("abcd[-1]").unwrap();
        let expected = Subscript(Box::new(Identifier("abcd".into())), -1);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_invalid_identifier() {
        let err = from_str("!").unwrap_err();
        assert_eq!("", err.to_string());
    }

    #[test]
    fn test_invalid_child() {
        let err = from_str("a..").unwrap_err();
        assert_eq!("", err.to_string());
    }

    #[test]
    fn test_invalid_subscript() {
        let err = from_str("a[b]").unwrap_err();
        assert_eq!("", err.to_string());
    }

    #[test]
    fn test_incomplete_subscript() {
        let err = from_str("a[0").unwrap_err();
        assert_eq!("", err.to_string());
    }

    #[test]
    fn test_invalid_postfix() {
        let err = from_str("a!b").unwrap_err();
        assert_eq!("", err.to_string());
    }
}
