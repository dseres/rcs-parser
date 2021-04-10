#![allow(dead_code)]

pub static CONTEXT: &str = "Diff";

use crate::DiffCommand;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{digit1, line_ending, multispace0, multispace1, one_of, space0},
    combinator::map,
    error::{context, VerboseError},
    multi::{count, many0},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

/// Parsing diff format.
///
/// GNU document RCS diff format in the [diffutils documentation](https://www.gnu.org/software/diffutils/manual/html_node/RCS.html#RCS).
///
/// Examples
/// ```ignore
/// use rcs_parser::{parse_diff_command, DiffCommand};
/// use nom::{
///     error::{ErrorKind, VerboseError, VerboseErrorKind},
///     Err,
/// };
/// assert_eq!(
///     Ok(("", DiffCommand::Delete(1, 2))),
///     parse_diff_command("d1 2\r\n")
/// );
/// assert_eq!(
///     Ok(("", DiffCommand::Delete(1, 2))),
///     parse_diff_command("\t d  1 \n 2\n")
/// );
///
/// assert_eq!(
///     Ok((
///         "",
///         DiffCommand::Add(23, vec!["First line added.".to_string(), "Second line added.".to_string()])
///     )),
///     parse_diff_command("a23 2\nFirst line added.\nSecond line added.\n")
/// );
///
/// assert_eq!(
///     Err(Err::Error(VerboseError {
///         errors: vec![
///             ("c2 3\n", VerboseErrorKind::Nom(ErrorKind::OneOf)),
///             ("c2 3\n", VerboseErrorKind::Context("Diff"))
///         ]
///     })),
///     parse_diff_command("c2 3\n")
/// );
/// ```
pub fn parse_diff_command(input: &str) -> IResult<&str, DiffCommand, VerboseError<&str>> {
    let (input, (command, position, length)) = context(
        CONTEXT,
        tuple((
            preceded(multispace0, one_of("ad")),
            preceded(
                multispace0,
                map(digit1, |s| u32::from_str_radix(s, 10).unwrap()),
            ),
            preceded(
                multispace1,
                terminated(
                    map(digit1, |s| u32::from_str_radix(s, 10).unwrap()),
                    pair(space0, line_ending),
                ),
            ),
        )),
    )(input)?;
    if command == 'a' {
        let (input, lines) = context(CONTEXT, count(parse_diff_line, length as usize))(input)?;
        Ok((input, DiffCommand::Add(position, lines)))
    } else {
        Ok((input, DiffCommand::Delete(position, length)))
    }
}

/// Parses one line from diff text.
///
/// Diff is part of a delimited '@string@', so it have to parse delimited '@@' too.
///
/// This function cannot read Mac style line endings. ("\r").
///
/// Examples
/// ```ignore
/// use rcs_parser::parse_diff_line;
/// use nom::{
///     error::{ErrorKind, VerboseError, VerboseErrorKind},
///     Err,
/// };
///
/// assert_eq!(Ok(("", "abc".to_string())), parse_diff_line("abc\n"));
/// assert_eq!(Ok(("", "abc 123".to_string())), parse_diff_line("abc 123\r\n"));
/// assert_eq!(Ok(("", "abc@abc".to_string())), parse_diff_line("abc@@abc\n"));
///
/// assert_eq!(
///     Err(Err::Error(VerboseError {
///         errors: vec![
///             ("", VerboseErrorKind::Nom(ErrorKind::CrLf)),
///             ("abc", VerboseErrorKind::Context( "Diff"))
///         ]
///     })),
///     parse_diff_line("abc")
/// );
///
/// assert_eq!(
///     Err(Err::Error(VerboseError {
///         errors: vec![
///             ("@abc\n", VerboseErrorKind::Nom(ErrorKind::CrLf)),
///             ("abc@abc\n", VerboseErrorKind::Context( "Diff"))
///         ]
///     })),
///     parse_diff_line("abc@abc\n")
/// );
/// ```
pub fn parse_diff_line(input: &str) -> IResult<&str, String, VerboseError<&str>> {
    context(
        CONTEXT,
        terminated(
            map(
                many0(alt((is_not("@\r\n"), map(tag("@@"), |_| "@")))),
                |v| v.concat(),
            ),
            line_ending,
        ),
    )(input)
}

#[cfg(test)]
mod test {

    use crate::DiffCommand;
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err,
    };

    #[test]
    fn parse_diff_line() {
        assert_eq!(
            Ok(("def", "abc".to_string())),
            super::parse_diff_line("abc\ndef")
        );
        assert_eq!(
            Ok(("def", "abc@abc".to_string())),
            super::parse_diff_line("abc@@abc\ndef")
        );
        assert_eq!(
            Ok(("def", "abc@@abc".to_string())),
            super::parse_diff_line("abc@@@@abc\ndef")
        );
        assert_eq!(
            Ok(("def", "abc".to_string())),
            super::parse_diff_line("abc\r\ndef")
        );
        assert_eq!(Ok(("", "@".to_string())), super::parse_diff_line("@@\n"));
        assert_eq!(Ok(("", "".to_string())), super::parse_diff_line("\n"));
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("@abc\n", VerboseErrorKind::Nom(ErrorKind::CrLf)),
                    ("abc@abc\n", VerboseErrorKind::Context(super::CONTEXT))
                ]
            })),
            super::parse_diff_line("abc@abc\n")
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("", VerboseErrorKind::Nom(ErrorKind::CrLf)),
                    ("abc", VerboseErrorKind::Context(super::CONTEXT))
                ]
            })),
            super::parse_diff_line("abc")
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("", VerboseErrorKind::Nom(ErrorKind::CrLf)),
                    ("", VerboseErrorKind::Context(super::CONTEXT))
                ]
            })),
            super::parse_diff_line("")
        );
    }

    #[test]
    fn parse_diff_command() {
        assert_eq!(
            Ok(("", DiffCommand::Delete(1, 2))),
            super::parse_diff_command("d1 2\r\n")
        );
        assert_eq!(
            Ok(("", DiffCommand::Delete(1, 2))),
            super::parse_diff_command("d  1 \n 2\n")
        );
        assert_eq!(
            Ok((
                "",
                DiffCommand::Add(1213, vec!["aaa".to_string(), "bbb".to_string()])
            )),
            super::parse_diff_command("a1213 2\naaa\nbbb\n")
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("c2 3\n", VerboseErrorKind::Nom(ErrorKind::OneOf)),
                    ("c2 3\n", VerboseErrorKind::Context(super::CONTEXT))
                ]
            })),
            super::parse_diff_command("c2 3\n")
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("", VerboseErrorKind::Nom(ErrorKind::CrLf)),
                    ("", VerboseErrorKind::Context(super::CONTEXT)),
                    ("", VerboseErrorKind::Nom(ErrorKind::Count)),
                    ("", VerboseErrorKind::Context(super::CONTEXT)),
                ]
            })),
            super::parse_diff_command("a2 3\n")
        );
    }
}
