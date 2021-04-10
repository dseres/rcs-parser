#![allow(dead_code)]

use nom::{
    bytes::complete::take_while1,
    error::{context, VerboseError},
    IResult,
};

///Checks if a character is special.
///
/// > special   ::=  "$" | "," | "." | ":" | ";" | "@"
///
/// Examples:
/// ```ignore
/// use rcs_parser::is_special_chars;
///
/// assert_eq!( true, is_special_chars('$'));
/// assert_eq!( true, is_special_chars(','));
/// assert_eq!( false, is_special_chars('!'));
/// ```
pub fn is_special_chars(c: char) -> bool {
    c == '$' || c == ',' || c == '.' || c == ':' || c == ';' || c == '@'
}

/// Checks if a character is visible. Visible graphic characters are (octal) codes 041–176 and 240–377.
///
/// Examples:
/// ```ignore
/// use rcs_parser::is_visible_char;
///
/// assert_eq!( true, is_visible_char('a'));
/// assert_eq!( true, is_visible_char('9'));
/// assert_eq!( true, is_visible_char('*'));
/// assert_eq!( true, is_visible_char('.'));
/// assert_eq!( true, is_visible_char('ö'));
/// assert_eq!( false, is_visible_char(' '));
/// ```
pub fn is_visible_char(c: char) -> bool {
    (0x21 as char) <= c && c <= (0x7e as char) || (0xA0 as char) <= c && c <= (0xff as char)
}

///Checks if a character is idchar.
///
/// > idchar    ::=  any visible graphic character except [special](is_special_chars)
///
/// Examples:
/// ```ignore
/// use rcs_parser::is_idchar;
///
/// assert_eq!( true, is_idchar('a'));
/// assert_eq!( true, is_idchar('9'));
/// assert_eq!( true, is_idchar('*'));
/// assert_eq!( false, is_idchar('.'));
/// assert_eq!( false, is_idchar(' '));
/// ```
pub fn is_idchar(c: char) -> bool {
    is_visible_char(c) && !is_special_chars(c)
}

/// parses a symbol
///
/// > sym       ::=  {idchar}+
///
/// Example:
/// ```ignore
/// use rcs_parser::parse_sym;
/// use nom::{
///     error::{Error, ErrorKind, VerboseError, VerboseErrorKind},
///     Err,
/// };
///
/// assert_eq!( Ok(("$zzz","abc123*")), parse_sym("abc123*$zzz"));
/// assert_eq!( Ok(("\t abc","ABC123!")), parse_sym("ABC123!\t abc"));
/// assert_eq!(  Err(Err::Error(VerboseError {
///     errors: vec![
///         (" abc", VerboseErrorKind::Nom(ErrorKind::TakeWhile1)),
///         (" abc", VerboseErrorKind::Context("sym"))
///     ]
///     })), parse_sym(" abc"));
/// ```
pub fn parse_sym(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context("sym", take_while1(is_idchar))(input)
}

/// parses a symbol
///
/// > sym       ::=  {idchar}+
///
/// Example:
/// ```ignore
/// use rcs_parser::parse_id;
/// use nom::{
///     error::{Error, ErrorKind, VerboseError, VerboseErrorKind},
///     Err,
/// };
///
/// assert_eq!( Ok(("@xyz","A.a.1.")), parse_id("A.a.1.@xyz"));
/// assert_eq!( Ok(("",".")), parse_id("."));
/// assert_eq!( Ok(("","A")), parse_id("A"));
/// assert_eq!(  Err(Err::Error(VerboseError {
///     errors: vec![
///         (" .abc", VerboseErrorKind::Nom(ErrorKind::TakeWhile1)),
///         (" .abc", VerboseErrorKind::Context("id"))
///     ]
/// })), parse_id(" .abc"));
/// ```
pub fn parse_id(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context("id", take_while1(|c| is_idchar(c) || c == '.'))(input)
}

#[cfg(test)]
mod test {
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err,
    };

    #[test]
    fn is_special_chars() {
        assert_eq!(true, super::is_special_chars('$'));
        assert_eq!(true, super::is_special_chars(','));
        assert_eq!(true, super::is_special_chars('.'));
        assert_eq!(true, super::is_special_chars(':'));
        assert_eq!(true, super::is_special_chars(';'));
        assert_eq!(true, super::is_special_chars('@'));

        assert_eq!(false, super::is_special_chars('\r'));
        assert_eq!(false, super::is_special_chars(' '));
        assert_eq!(false, super::is_special_chars('G'));
        assert_eq!(false, super::is_special_chars('8'));
        assert_eq!(false, super::is_special_chars('á'));
    }

    #[test]
    fn id_idchar() {
        assert_eq!(true, super::is_idchar('f'));
        assert_eq!(true, super::is_idchar('9'));
        assert_eq!(true, super::is_idchar('F'));
        assert_eq!(true, super::is_idchar('*'));
        assert_eq!(true, super::is_idchar('~'));
        assert_eq!(true, super::is_idchar('!'));
        assert_eq!(true, super::is_idchar('á'));

        assert_eq!(false, super::is_idchar('$'));
        assert_eq!(false, super::is_idchar(' '));
        assert_eq!(false, super::is_idchar('\u{007f}'));
        assert_eq!(false, super::is_idchar(' '));
        assert_eq!(false, super::is_idchar('\n'));
    }

    #[test]
    fn parse_sym() {
        assert_eq!(Ok(("$zzz", "abc123*")), super::parse_sym("abc123*$zzz"));
        assert_eq!(Ok((" ~~~", "XZY-_")), super::parse_sym("XZY-_ ~~~"));
        assert_eq!(Ok(("\t abc", "abc123!")), super::parse_sym("abc123!\t abc"));
        assert_eq!(Ok(("", "abc123*é")), super::parse_sym("abc123*é"));
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    (" abc", VerboseErrorKind::Nom(ErrorKind::TakeWhile1)),
                    (" abc", VerboseErrorKind::Context("sym"))
                ]
            })),
            super::parse_sym(" abc")
        );
    }

    #[test]
    fn parse_id() {
        assert_eq!(Ok(("@xyz", "A.a.1.")), super::parse_id("A.a.1.@xyz"));
        assert_eq!(Ok(("", ".")), super::parse_id("."));
        assert_eq!(Ok(("", "A")), super::parse_id("A"));
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    (" .abc", VerboseErrorKind::Nom(ErrorKind::TakeWhile1)),
                    (" .abc", VerboseErrorKind::Context("id"))
                ]
            })),
            super::parse_id(" .abc")
        );
    }
}
