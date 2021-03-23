extern crate jemallocator;
extern crate nom;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

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
/// extern crate rcs_parser;
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
        map(
            preceded(
                tag("@"),
                terminated(
                    map(many0(alt((is_not("@"), map(tag("@@"), |_| "@")))), |v| {
                        v.concat()
                    }),
                    tag("@"),
                ),
            ),
            |s| s.to_string(),
        ),
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
        assert_eq!(Ok(("", "".to_string())), crate::parse_string("@@"));
        assert_eq!(
            Ok(("xyz", "abc".to_string())),
            crate::parse_string("@abc@xyz")
        );
        assert_eq!(Ok(("xyz", "@".to_string())), crate::parse_string("@@@@xyz"));
        assert_eq!(
            Ok(("xyz", "abc@def".to_string())),
            crate::parse_string("@abc@@def@xyz")
        );
        assert_eq!(
            Ok(("xyz", "abc@def@@ghi".to_string())),
            crate::parse_string("@abc@@def@@@@ghi@xyz")
        );
        assert_eq!(
            Err(Err::Error(VerboseError{ errors:  vec![("zzz", VerboseErrorKind::Nom(ErrorKind::Tag)), ("zzz", VerboseErrorKind::Context("string"))]})),
            crate::parse_string("zzz")
        );
        assert_eq!(
            Err(Err::Error(VerboseError{ errors:  vec![("zzz@", VerboseErrorKind::Nom(ErrorKind::Tag)), ("zzz@", VerboseErrorKind::Context("string"))]})),
            crate::parse_string("zzz@")
        );
        assert_eq!(
            Err(Err::Error(VerboseError{ errors:  vec![("", VerboseErrorKind::Nom(ErrorKind::Tag)), ("@zzz", VerboseErrorKind::Context("string"))]})),
            crate::parse_string("@zzz")
        );
    }
}
