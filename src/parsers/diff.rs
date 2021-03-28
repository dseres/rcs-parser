#![allow(dead_code)]

pub static CONTEXT: &str = "Diff";

use crate::*;
use nom::{
    character::complete::{
        digit1, line_ending, multispace0, multispace1, not_line_ending, one_of, space0,
    },
    combinator::map,
    error::{context, VerboseError},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

/// Parsing diff format. 
/// 
/// GNU document RCS diff format in the [diffutils documentation](https://www.gnu.org/software/diffutils/manual/html_node/RCS.html#RCS).
/// 
/// Examples
/// ```rust
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
///     parse_diff_command("d  1 \n 2\n")
/// );
/// assert_eq!(
///     Ok((
///         "",
///         DiffCommand::Add(1213, vec!["aaa".to_string(), "bbb".to_string()])
///     )),
///     parse_diff_command("a1213 2\naaa\nbbb\n")
/// );
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
            one_of("ad"),
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
        let (input, lines) = parse_diff_lines(input, length)?;
        Ok((input, DiffCommand::Add(position, lines)))
    } else {
        Ok((input, DiffCommand::Delete(position, length)))
    }
}

/// Reads n lines from a diff string
/// 
/// Examples
/// ```rust
/// use rcs_parser::parse_diff_lines;
/// use nom::{
///     error::{ErrorKind, VerboseError, VerboseErrorKind},
///     Err,
/// };
/// assert_eq!(
///     Ok(("", vec!["first line".to_string(), "second line".to_string()])),
///     parse_diff_lines("first line\nsecond line\n", 2)
/// );
/// assert_eq!(
///     Ok(("ghi\r\n", vec!["first line with windows style line ending".to_string(), "second line with windows style line ending".to_string()])),
///     parse_diff_lines("first line with windows style line ending\r\nsecond line with windows style line ending\r\nghi\r\n", 2)
/// );
/// assert_eq!(
///     Err(Err::Error(VerboseError {
///         errors: vec![
///             ("", VerboseErrorKind::Nom(ErrorKind::CrLf)),
///             ("third line without line ending", VerboseErrorKind::Context("Diff"))
///         ]
///     })),
///     parse_diff_lines("first line\nsecond line\nthird line without line ending", 3)
/// );
/// ``` 
pub fn parse_diff_lines(
    input: &str,
    line_count: u32,
) -> IResult<&str, Vec<String>, VerboseError<&str>> {
    let mut input = input;
    let mut lines = Vec::<String>::new();
    for _ in 1..=line_count {
        let (next_input, line) = parse_diff_line(input)?;
        lines.push(line.to_string());
        input = next_input;
    }
    Ok((input, lines))
}

/// Parses one line from input.
///
/// Examples
/// ```rust
/// use rcs_parser::parse_diff_line;
/// use nom::{
///     error::{ErrorKind, VerboseError, VerboseErrorKind},
///     Err,
/// };
///
/// assert_eq!(Ok(("", "abc")), parse_diff_line("abc\n"));
/// assert_eq!(Ok(("", "abc 123")), parse_diff_line("abc 123\r\n"));
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
/// ```
pub fn parse_diff_line(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(CONTEXT, terminated(not_line_ending, line_ending))(input)
}

#[cfg(test)]
mod test {

    use crate::{parsers::diff::CONTEXT, DiffCommand};
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err,
    };

    #[test]
    fn parse_diff_line() {
        assert_eq!(Ok(("def", "abc")), crate::parse_diff_line("abc\ndef"));
        assert_eq!(Ok(("def", "abc")), crate::parse_diff_line("abc\r\ndef"));
        assert_eq!(Ok(("", "")), crate::parse_diff_line("\n"));
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("", VerboseErrorKind::Nom(ErrorKind::CrLf)),
                    ("abc", VerboseErrorKind::Context(CONTEXT))
                ]
            })),
            crate::parse_diff_line("abc")
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("", VerboseErrorKind::Nom(ErrorKind::CrLf)),
                    ("", VerboseErrorKind::Context(CONTEXT))
                ]
            })),
            crate::parse_diff_line("")
        );
    }

    #[test]
    fn parse_diff_lines() {
        assert_eq!(
            Ok(("", vec!["abc".to_string(), "def".to_string()])),
            crate::parse_diff_lines("abc\ndef\n", 2)
        );
        assert_eq!(
            Ok(("ghi\r\n", vec!["abc".to_string(), "def".to_string()])),
            crate::parse_diff_lines("abc\r\ndef\r\nghi\r\n", 2)
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("", VerboseErrorKind::Nom(ErrorKind::CrLf)),
                    ("g", VerboseErrorKind::Context(CONTEXT))
                ]
            })),
            crate::parse_diff_lines("abc\ndef\ng", 3)
        );
    }

    #[test]
    fn parse_diff_command() {
        assert_eq!(
            Ok(("", DiffCommand::Delete(1, 2))),
            crate::parse_diff_command("d1 2\r\n")
        );
        assert_eq!(
            Ok(("", DiffCommand::Delete(1, 2))),
            crate::parse_diff_command("d  1 \n 2\n")
        );
        assert_eq!(
            Ok((
                "",
                DiffCommand::Add(1213, vec!["aaa".to_string(), "bbb".to_string()])
            )),
            crate::parse_diff_command("a1213 2\naaa\nbbb\n")
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("c2 3\n", VerboseErrorKind::Nom(ErrorKind::OneOf)),
                    ("c2 3\n", VerboseErrorKind::Context(CONTEXT))
                ]
            })),
            crate::parse_diff_command("c2 3\n")
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("", VerboseErrorKind::Nom(ErrorKind::Digit)),
                    ("a2 ", VerboseErrorKind::Context(CONTEXT))
                ]
            })),
            crate::parse_diff_command("a2 ")
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("", VerboseErrorKind::Nom(ErrorKind::CrLf)),
                    ("", VerboseErrorKind::Context(CONTEXT))
                ]
            })),
            crate::parse_diff_command("a2 3\n")
        );
    }
}
