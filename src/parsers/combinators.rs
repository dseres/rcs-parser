#![warn(dead_code)]

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::opt,
    error::{context, ContextError, ParseError},
    multi::many0,
    sequence::{delimited, preceded},
    AsChar, Compare, IResult, InputTake, InputTakeAtPosition, Parser,
};

pub fn parse_value<I: Clone, O, E: ParseError<I>, F>(
    ctx: &'static str,
    key: &'static str,
    f: F,
) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: Parser<I, O, E>,
    I: InputTakeAtPosition + Compare<&'static str> + InputTake,
    <I as nom::InputTakeAtPosition>::Item: AsChar + Clone,
    E: ContextError<I>,
{
    context(
        ctx,
        delimited(
            preceded(multispace0, tag(key)),
            preceded(multispace0, f),
            preceded(multispace0, tag(";")),
        ),
    )
}

pub fn parse_value_opt<I: Clone, O, E: ParseError<I>, F>(
    ctx: &'static str,
    key: &'static str,
    f: F,
) -> impl FnMut(I) -> IResult<I, Option<O>, E>
where
    F: Parser<I, O, E>,
    I: InputTakeAtPosition + Compare<&'static str> + InputTake,
    <I as nom::InputTakeAtPosition>::Item: AsChar + Clone,
    E: ContextError<I>,
{
    context(
        ctx,
        delimited(
            preceded(multispace0, tag(key)),
            preceded(multispace0, opt(f)),
            preceded(multispace0, tag(";")),
        ),
    )
}

pub fn parse_value_all_opt<I: Clone, O, E: ParseError<I>, F>(
    ctx: &'static str,
    key: &'static str,
    f: F,
) -> impl FnMut(I) -> IResult<I, Option<O>, E>
where
    F: Parser<I, O, E>,
    I: InputTakeAtPosition + Compare<&'static str> + InputTake,
    <I as nom::InputTakeAtPosition>::Item: AsChar + Clone,
    E: ContextError<I>,
{
    context(
        ctx,
        opt(delimited(
            preceded(multispace0, tag(key)),
            preceded(multispace0, f),
            preceded(multispace0, tag(";")),
        )),
    )
}

pub fn parse_value_many0<I: Clone, O, E: ParseError<I>, F>(
    ctx: &'static str,
    key: &'static str,
    f: F,
) -> impl FnMut(I) -> IResult<I, Vec<O>, E>
where
    F: Parser<I, O, E>,
    I: InputTakeAtPosition + Compare<&'static str> + InputTake + PartialEq,
    <I as nom::InputTakeAtPosition>::Item: AsChar + Clone,
    E: ContextError<I>,
{
    context(
        ctx,
        delimited(
            preceded(multispace0, tag(key)),
            many0(preceded(multispace1, f)),
            preceded(multispace0, tag(";")),
        ),
    )
}

#[cfg(test)]
mod test {
    use crate::{parsers::*, *};
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err,
    };

    #[test]
    fn parse_value() {
        let mut parser = super::parse_value("context", "num", parse_num);

        let input = " num 1.2.3;";
        assert_eq!(Ok(("", num!(1, 2, 3))), parser(input));

        let input = "non num";
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("non num", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("non num", VerboseErrorKind::Context("context"))
                ]
            })),
            parser(input)
        );

        let input = "num numanumaje";
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("numanumaje", VerboseErrorKind::Nom(ErrorKind::Digit)),
                    ("numanumaje", VerboseErrorKind::Context("Num")),
                    ("num numanumaje", VerboseErrorKind::Context("context"))
                ]
            })),
            parser(input)
        );
    }

    #[test]
    fn parse_value_opt() {
        let mut parser = super::parse_value_opt("context", "optional", parse_id);

        let input = " optional ;";
        assert_eq!(Ok(("", None)), parser(input));

        let input = " optional value;";
        assert_eq!(Ok(("", Some("value"))), parser(input));

        let input = "bad tag";
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("bad tag", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("bad tag", VerboseErrorKind::Context("context"))
                ]
            })),
            parser(input)
        );
    }

    #[test]
    fn parse_value_all_opt() {
        let mut parser = super::parse_value_all_opt("context", "optional", parse_id);

        let input = " optional ;";
        assert_eq!(Ok((" optional ;", None)), parser(input));

        let input = "optional value;";
        assert_eq!(Ok(("", Some("value"))), parser(input));

        let input = "bad tag";
        assert_eq!(Ok(("bad tag", None)), parser(input));
    }

    #[test]
    fn parse_value_many0() {
        let mut parser = super::parse_value_many0("context", "many0", parse_id);

        let input = "many0;";
        assert_eq!(Ok(("", vec![])), parser(input));

        let input = "many0 abc;";
        assert_eq!(Ok(("", vec!["abc"])), parser(input));

        let input = "many0 abc def;";
        assert_eq!(Ok(("", vec!["abc", "def"])), parser(input));

        let input = "bad tag";
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("bad tag", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("bad tag", VerboseErrorKind::Context("context"))
                ]
            })),
            parser(input)
        );
    }
}
