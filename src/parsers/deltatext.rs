#![allow(dead_code)]

use crate::{parsers::*, *};
use nom::{
    bytes::complete::tag,
    character::complete::multispace1,
    combinator::map,
    error::{context, VerboseError},
    multi::many0,
    sequence::{preceded, terminated, tuple},
    IResult,
};

/// holds differences between revisions.
#[derive(Debug, PartialEq, Clone)]
pub struct DeltaText {
    ///The revision number
    pub num: Num,
    ///Commit log
    pub log: String,
    ///Differences between this and its parent revision
    pub text: Text,
}

/// Parsing deltatext
///
/// Grammar of deltatext is:
/// > deltatext ::=  num
/// >                "log"   string
/// >                "text"  string
///
/// Example:
/// ```ignore
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
    let (input, (num, log, text)) = context(
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
                    preceded(
                        tag("@"),
                        map(terminated(many0(parse_diff_command), tag("@")), |v| {
                            Text::Diff(v)
                        }),
                    ),
                ),
            ),
        )),
    )(input)?;
    Ok((input, DeltaText { num, log, text }))
}

pub fn parse_deltatext_head(input: &str) -> IResult<&str, DeltaText, VerboseError<&str>> {
    let (input, (num, log, text)) = context(
        "DeltaText",
        tuple((
            parse_num,
            preceded(
                preceded(multispace1, tag("log")),
                preceded(multispace1, parse_string),
            ),
            preceded(
                preceded(multispace1, tag("text")),
                preceded(multispace1, map(parse_string, Text::Head)),
            ),
        )),
    )(input)?;
    Ok((input, DeltaText { num, log, text }))
}

#[cfg(test)]
mod test {
    use crate::*;
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
                super::DeltaText {
                    num: Num {
                        numbers: vec![1, 1]
                    },
                    log: "Initial revision\n".to_string(),
                    text: Text::Diff(vec![
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
                    ])
                }
            )),
            super::parse_deltatext(delta_str)
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
            super::parse_deltatext(delta_str)
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
            super::parse_deltatext(delta_str)
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
            super::parse_deltatext(delta_str)
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
            super::parse_deltatext(delta_str)
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
            super::parse_deltatext(delta_str)
        );
    }

    #[test]
    fn parse_deltatext_head() {
        let delta_str = r#"2.1
log
@lao back
@
text
@The Way that can be told of is not the eternal Way;
The name that can be named is not the eternal name.
The Nameless is the origin of Heaven and Earth;
The Named is the mother of all things.
Therefore let there always be non-being,
  so we may see their subtlety,
And let there always be being,
  so we may see their outcome.
The two are the same,
But after they are produced,
  they have different names.
@"#;
        let text = String::from(
            r#"The Way that can be told of is not the eternal Way;
The name that can be named is not the eternal name.
The Nameless is the origin of Heaven and Earth;
The Named is the mother of all things.
Therefore let there always be non-being,
  so we may see their subtlety,
And let there always be being,
  so we may see their outcome.
The two are the same,
But after they are produced,
  they have different names.
"#,
        );
        assert_eq!(
            Ok((
                "",
                super::DeltaText {
                    num: Num {
                        numbers: vec![2, 1]
                    },
                    log: "lao back\n".to_string(),
                    text: Text::Head(text),
                }
            )),
            super::parse_deltatext_head(delta_str)
        );
    }
}
