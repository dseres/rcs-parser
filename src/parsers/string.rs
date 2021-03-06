#![allow(dead_code)]

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    combinator::map,
    error::{context, VerboseError},
    multi::many0,
    sequence::delimited,
    IResult,
};

/// Parsing string value in RCS file.
///
/// String grammar is:
///
/// > string    ::=  "@" { any character, with @ doubled }* "@"
///
/// Example:
/// ```ignore
/// use rcs_parser::parse_string;
/// use nom::{
///     error::{Error, ErrorKind, VerboseError, VerboseErrorKind},
///     Err,
/// };
///
/// assert_eq!(
///     Ok(("Other input.", "This is a test string.".to_string())),
///     parse_string("@This is a test string.@Other input."));
/// assert_eq!(
///     Ok(("", "String having an '@' inside.".to_string())),
///     parse_string("@String having an '@@' inside.@"));
/// assert_eq!(
///     Err(Err::Error(VerboseError{ errors:  vec![("", VerboseErrorKind::Nom(ErrorKind::Tag)), ("@zzz", VerboseErrorKind::Context("string"))]})),
///     parse_string("@zzz"));
/// assert_eq!(
///     Err(Err::Error(VerboseError{ errors:  vec![("zzz@", VerboseErrorKind::Nom(ErrorKind::Tag)), ("zzz@", VerboseErrorKind::Context("string"))]})),
///     parse_string("zzz@"));
/// assert_eq!(
///     Err(Err::Error(VerboseError{ errors:  vec![("zz@@z", VerboseErrorKind::Nom(ErrorKind::Tag)), ("zz@@z", VerboseErrorKind::Context("string"))]})),
///     parse_string("zz@@z"));
/// ```
pub fn parse_string(input: &str) -> IResult<&str, String, VerboseError<&str>> {
    context(
        "string",
        delimited(
            tag("@"),
            map(many0(alt((is_not("@"), map(tag("@@"), |_| "@")))), |v| {
                v.concat()
            }),
            tag("@"),
        ),
    )(input)
}

pub fn parse_intstring(input: &str) -> IResult<&str, String, VerboseError<&str>> {
    context(
        "intstring",
        delimited(tag("@"), map(take_until("@"), String::from), tag("@")),
    )(input)
}

#[cfg(test)]
mod tests {

    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err,
    };

    #[test]
    fn parse_string() {
        assert_eq!(Ok(("", "".to_string())), super::parse_string("@@"));
        assert_eq!(
            Ok(("xyz", "abc".to_string())),
            super::parse_string("@abc@xyz")
        );
        assert_eq!(Ok(("xyz", "@".to_string())), super::parse_string("@@@@xyz"));
        assert_eq!(
            Ok(("xyz", "abc@def".to_string())),
            super::parse_string("@abc@@def@xyz")
        );
        assert_eq!(
            Ok(("xyz", "abc@def@@ghi".to_string())),
            super::parse_string("@abc@@def@@@@ghi@xyz")
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("zzz", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("zzz", VerboseErrorKind::Context("string"))
                ]
            })),
            super::parse_string("zzz")
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("zzz@", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("zzz@", VerboseErrorKind::Context("string"))
                ]
            })),
            super::parse_string("zzz@")
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("@zzz", VerboseErrorKind::Context("string"))
                ]
            })),
            super::parse_string("@zzz")
        );
    }

    #[test]
    fn parse_intstring() {
        assert_eq!(Ok(("", "".to_string())), super::parse_intstring("@@"));
        assert_eq!(
            Ok(("xyz", "abc".to_string())),
            super::parse_intstring("@abc@xyz")
        );
        assert_eq!(
            Ok(("@xyz@", "abc".to_string())),
            super::parse_intstring("@abc@@xyz@")
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("zzz", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("zzz", VerboseErrorKind::Context("intstring"))
                ]
            })),
            super::parse_intstring("zzz")
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("zzz", VerboseErrorKind::Nom(ErrorKind::TakeUntil)),
                    ("@zzz", VerboseErrorKind::Context("intstring"))
                ]
            })),
            super::parse_intstring("@zzz")
        );
    }
}
