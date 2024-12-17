use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{char, digit1, space0},
    combinator::{map, map_res, opt, recognize},
    error::ErrorKind,
    sequence::{delimited, pair, preceded},
    Err, IResult,
};

use crate::path::Expression;

pub(crate) fn from_str(input: &str) -> Result<Expression, ErrorKind> {
    match ident(input) {
        Ok((mut rem, mut expr)) => {
            while !rem.is_empty() {
                match postfix(expr)(rem) {
                    Ok((rem_, expr_)) => {
                        rem = rem_;
                        expr = expr_;
                    }

                    // Forward Incomplete and Error
                    result => {
                        return result.map(|(_, o)| o).map_err(to_error_kind);
                    }
                }
            }

            Ok(expr)
        }

        // Forward Incomplete and Error
        result => result.map(|(_, o)| o).map_err(to_error_kind),
    }
}

fn ident(i: &str) -> IResult<&str, Expression> {
    map(raw_ident, Expression::Identifier)(i)
}

fn postfix<'a>(expr: Expression) -> impl FnMut(&'a str) -> IResult<&'a str, Expression> {
    let e2 = expr.clone();
    let child = map(preceded(tag("."), raw_ident), move |id| {
        Expression::Child(Box::new(expr.clone()), id)
    });

    let subscript = map(delimited(char('['), integer, char(']')), move |num| {
        Expression::Subscript(Box::new(e2.clone()), num)
    });

    alt((child, subscript))
}

fn raw_ident(i: &str) -> IResult<&str, String> {
    map(
        is_a(
            "abcdefghijklmnopqrstuvwxyz \
         ABCDEFGHIJKLMNOPQRSTUVWXYZ \
         0123456789 \
         _-",
        ),
        ToString::to_string,
    )(i)
}

fn integer(i: &str) -> IResult<&str, isize> {
    map_res(
        delimited(space0, recognize(pair(opt(tag("-")), digit1)), space0),
        FromStr::from_str,
    )(i)
}

fn to_error_kind(e: Err<nom::error::Error<&str>>) -> ErrorKind {
    match e {
        Err::Incomplete(_) => ErrorKind::Complete,
        Err::Failure(e) | Err::Error(e) => e.code,
    }
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
}
