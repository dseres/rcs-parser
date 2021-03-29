#![allow(dead_code)]

use crate::*;
use nom::{
    bytes::complete::tag,
    character::complete::multispace1,
    error::{context, VerboseError},
    multi::many0,
    sequence::{preceded, terminated, tuple},
    IResult,
};

/// Parsing deltatext
///
/// Grammar of deltatext is:
/// > deltatext ::=  num
/// >                "log"   string
/// >                "text"  string
///
/// Example:
/// ```rust
/// use rcs_parser::{parse_deltatext, DeltaText, DiffCommand, Num};
///
/// let delta_str = r#"1.1
/// log
/// @Initial revision
/// @
/// text
/// @a0 2
/// The Way that can be told of is not the eternal Way;
/// The name that can be named is not the eternal name.
/// d2 2
/// a3 1
/// The Named is the mother of all things.
/// d11 3
/// @"#;
///
/// assert_eq!(
///     Ok((
///         "",
///         DeltaText {
///             num: Num {
///                 numbers: vec![1, 1]
///             },
///             log: "Initial revision\n".to_string(),
///             diff: vec![
///                 DiffCommand::Add(
///                     0,
///                     vec![
///                         "The Way that can be told of is not the eternal Way;".to_string(),
///                         "The name that can be named is not the eternal name.".to_string()
///                     ]
///                 ),
///                 DiffCommand::Delete(2, 2),
///                 DiffCommand::Add(
///                     3,
///                     vec!["The Named is the mother of all things.".to_string()]
///                 ),
///                 DiffCommand::Delete(11, 3)
///             ]
///         }
///     )),
///     parse_deltatext(delta_str)
/// );
///
/// ```
pub fn parse_deltatext(input: &str) -> IResult<&str, DeltaText, VerboseError<&str>> {
    let (input, (num, log, diff)) = context(
        "DeltaText",
        tuple((
            parse_num,
            preceded(
                preceded(multispace1, tag("log")),
                preceded(multispace1, parse_string),
            ),
            preceded(
                preceded(multispace1, tag("text")),
                preceded(
                    multispace1,
                    preceded(tag("@"), terminated(many0(parse_diff_command), tag("@"))),
                ),
            ),
        )),
    )(input)?;
    Ok((input, DeltaText { num, log, diff }))
}

#[cfg(test)]
mod test {
    use crate::{parsers::deltatext, DeltaText, DiffCommand, Num};
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err,
    };

    #[test]
    fn parse_deltatext() {
        let delta_str = r#"1.1
log
@Initial revision
@
text
@a0 2
The Way that can be told of is not the eternal Way;
The name that can be named is not the eternal name.
d2 2
a3 1
The Named is the mother of all things.
d11 3
@"#;
        assert_eq!(
            Ok((
                "",
                DeltaText {
                    num: Num {
                        numbers: vec![1, 1]
                    },
                    log: "Initial revision\n".to_string(),
                    diff: vec![
                        DiffCommand::Add(
                            0,
                            vec![
                                "The Way that can be told of is not the eternal Way;".to_string(),
                                "The name that can be named is not the eternal name.".to_string()
                            ]
                        ),
                        DiffCommand::Delete(2, 2),
                        DiffCommand::Add(
                            3,
                            vec!["The Named is the mother of all things.".to_string()]
                        ),
                        DiffCommand::Delete(11, 3)
                    ]
                }
            )),
            deltatext::parse_deltatext(delta_str)
        );
    }

    #[test]
    fn parse_deltatext2() {
        let delta_str = r#"
log
@Initial revision
@
text
@a0 2
The Way that can be told of is not the eternal Way;
The name that can be named is not the eternal name.
d2 2
a3 1
The Named is the mother of all things.
d11 3
@"#;
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    (delta_str, VerboseErrorKind::Nom(ErrorKind::Digit)),
                    (delta_str, VerboseErrorKind::Context("Num")),
                    (delta_str, VerboseErrorKind::Context("DeltaText")),
                ]
            })),
            deltatext::parse_deltatext(delta_str)
        );
    }

    #[test]
    fn parse_deltatext3() {
        let delta_str = r#"1.1
@Initial revision
@
text
@@"#;
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    (
                        "@Initial revision\n@\ntext\n@@",
                        VerboseErrorKind::Nom(ErrorKind::Tag)
                    ),
                    (delta_str, VerboseErrorKind::Context("DeltaText"))
                ]
            })),
            deltatext::parse_deltatext(delta_str)
        );

        let delta_str = r#"1.1
log
text
@@"#;
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("text\n@@", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("text\n@@", VerboseErrorKind::Context("string")),
                    (delta_str, VerboseErrorKind::Context("DeltaText")),
                ]
            })),
            deltatext::parse_deltatext(delta_str)
        );

        let delta_str = r#"1.1
log @@
@@"#;
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("@@", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    (delta_str, VerboseErrorKind::Context("DeltaText")),
                ]
            })),
            deltatext::parse_deltatext(delta_str)
        );

        let delta_str = r#"1.1
log @@
text "#;
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    (delta_str, VerboseErrorKind::Context("DeltaText")),
                ]
            })),
            deltatext::parse_deltatext(delta_str)
        );
    }
}
