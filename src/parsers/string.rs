#![allow(dead_code)]

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::map,
    error::{context, VerboseError},
    multi::many0,
    sequence::{preceded, terminated},
    IResult,
};

/// Parsing string value in RCS file.
///
/// String grammar is:
///
/// > string    ::=  "@" { any character, with @ doubled }* "@"
///
/// Example:
/// ```rust
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
#[allow(dead_code)]
pub fn parse_string(input: &str) -> IResult<&str, String, VerboseError<&str>> {
    context(
        "string",
        preceded(
            tag("@"),
            terminated(
                map(many0(alt((is_not("@"), map(tag("@@"), |_| "@")))), |v| {
                    v.concat()
                }),
                tag("@"),
            ),
        ),
    )(input)
}

#[cfg(test)]
mod tests {

    use crate::parsers::string;
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err,
    };

    #[test]
    fn parse_string() {
        assert_eq!(Ok(("", "".to_string())), string::parse_string("@@"));
        assert_eq!(
            Ok(("xyz", "abc".to_string())),
            string::parse_string("@abc@xyz")
        );
        assert_eq!(Ok(("xyz", "@".to_string())), string::parse_string("@@@@xyz"));
        assert_eq!(
            Ok(("xyz", "abc@def".to_string())),
            string::parse_string("@abc@@def@xyz")
        );
        assert_eq!(
            Ok(("xyz", "abc@def@@ghi".to_string())),
            string::parse_string("@abc@@def@@@@ghi@xyz")
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("zzz", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("zzz", VerboseErrorKind::Context("string"))
                ]
            })),
            string::parse_string("zzz")
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("zzz@", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("zzz@", VerboseErrorKind::Context("string"))
                ]
            })),
            string::parse_string("zzz@")
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("@zzz", VerboseErrorKind::Context("string"))
                ]
            })),
            string::parse_string("@zzz")
        );
    }
}
