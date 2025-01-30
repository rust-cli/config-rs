use std::str::FromStr;

use winnow::ascii::digit1;
use winnow::ascii::space0;
use winnow::combinator::cut_err;
use winnow::combinator::dispatch;
use winnow::combinator::fail;
use winnow::combinator::opt;
use winnow::combinator::repeat;
use winnow::combinator::seq;
use winnow::error::ContextError;
use winnow::error::ParseError;
use winnow::error::StrContext;
use winnow::error::StrContextValue;
use winnow::prelude::*;
use winnow::token::any;
use winnow::token::take_while;

use crate::path::Expression;
use crate::path::Postfix;

pub(crate) fn from_str(input: &str) -> Result<Expression, ParseError<&str, ContextError>> {
    path.parse(input)
}

fn path(i: &mut &str) -> ModalResult<Expression> {
    let root = ident.parse_next(i)?;
    let postfix = repeat(0.., postfix).parse_next(i)?;
    let expr = Expression { root, postfix };
    Ok(expr)
}

fn postfix(i: &mut &str) -> ModalResult<Postfix> {
    dispatch! {any;
        '[' => cut_err(
            seq!(
                integer.map(Postfix::Index),
                _: ']'.context(StrContext::Expected(StrContextValue::CharLiteral(']'))),
            )
                .map(|(i,)| i)
                .context(StrContext::Label("subscript"))
        ),
        '.' => cut_err(ident.map(Postfix::Key)),
        _ => cut_err(
            fail
                .context(StrContext::Label("postfix"))
                .context(StrContext::Expected(StrContextValue::CharLiteral('[')))
                .context(StrContext::Expected(StrContextValue::CharLiteral('.')))
        ),
    }
    .parse_next(i)
}

fn ident(i: &mut &str) -> ModalResult<String> {
    take_while(1.., ('a'..='z', 'A'..='Z', '0'..='9', '_', '-'))
        .map(ToOwned::to_owned)
        .context(StrContext::Label("identifier"))
        .context(StrContext::Expected(StrContextValue::Description(
            "ASCII alphanumeric",
        )))
        .context(StrContext::Expected(StrContextValue::CharLiteral('_')))
        .context(StrContext::Expected(StrContextValue::CharLiteral('-')))
        .parse_next(i)
}

fn integer(i: &mut &str) -> ModalResult<isize> {
    seq!(
        _: space0,
        (opt('-'), digit1).take().try_map(FromStr::from_str),
        _: space0
    )
    .context(StrContext::Expected(StrContextValue::Description(
        "integer",
    )))
    .map(|(i,)| i)
    .parse_next(i)
}

#[cfg(test)]
mod test {
    use snapbox::prelude::*;
    use snapbox::{assert_data_eq, str};

    use super::*;

    #[test]
    fn test_id() {
        let parsed: Expression = from_str("abcd").unwrap();
        assert_data_eq!(
            parsed.to_debug(),
            str![[r#"
Expression {
    root: "abcd",
    postfix: [],
}

"#]]
        );
    }

    #[test]
    fn test_id_dash() {
        let parsed: Expression = from_str("abcd-efgh").unwrap();
        assert_data_eq!(
            parsed.to_debug(),
            str![[r#"
Expression {
    root: "abcd-efgh",
    postfix: [],
}

"#]]
        );
    }

    #[test]
    fn test_child() {
        let parsed: Expression = from_str("abcd.efgh").unwrap();
        assert_data_eq!(
            parsed.to_debug(),
            str![[r#"
Expression {
    root: "abcd",
    postfix: [
        Key(
            "efgh",
        ),
    ],
}

"#]]
        );

        let parsed: Expression = from_str("abcd.efgh.ijkl").unwrap();
        assert_data_eq!(
            parsed.to_debug(),
            str![[r#"
Expression {
    root: "abcd",
    postfix: [
        Key(
            "efgh",
        ),
        Key(
            "ijkl",
        ),
    ],
}

"#]]
        );
    }

    #[test]
    fn test_subscript() {
        let parsed: Expression = from_str("abcd[12]").unwrap();
        assert_data_eq!(
            parsed.to_debug(),
            str![[r#"
Expression {
    root: "abcd",
    postfix: [
        Index(
            12,
        ),
    ],
}

"#]]
        );
    }

    #[test]
    fn test_subscript_neg() {
        let parsed: Expression = from_str("abcd[-1]").unwrap();
        assert_data_eq!(
            parsed.to_debug(),
            str![[r#"
Expression {
    root: "abcd",
    postfix: [
        Index(
            -1,
        ),
    ],
}

"#]]
        );
    }

    #[test]
    fn test_invalid_identifier() {
        let err = from_str("!").unwrap_err();
        assert_data_eq!(
            err.to_string(),
            str![[r#"
!
^
invalid identifier
expected ASCII alphanumeric, `_`, `-`
"#]]
        );
    }

    #[test]
    fn test_invalid_child() {
        let err = from_str("a..").unwrap_err();
        assert_data_eq!(
            err.to_string(),
            str![[r#"
a..
  ^
invalid identifier
expected ASCII alphanumeric, `_`, `-`
"#]]
        );
    }

    #[test]
    fn test_invalid_subscript() {
        let err = from_str("a[b]").unwrap_err();
        assert_data_eq!(
            err.to_string(),
            str![[r#"
a[b]
  ^
invalid subscript
expected integer
"#]]
        );
    }

    #[test]
    fn test_incomplete_subscript() {
        let err = from_str("a[0").unwrap_err();
        assert_data_eq!(
            err.to_string(),
            str![[r#"
a[0
   ^
invalid subscript
expected `]`
"#]]
        );
    }

    #[test]
    fn test_invalid_postfix() {
        let err = from_str("a!b").unwrap_err();
        assert_data_eq!(
            err.to_string(),
            str![[r#"
a!b
  ^
invalid postfix
expected `[`, `.`
"#]]
        );
    }
}
