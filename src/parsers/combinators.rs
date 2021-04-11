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

// fn parse_head(input: &str) -> IResult<&str, Num, VerboseError<&str>> {
//     context(
//         CONTEXT,
//         delimited(
//             preceded(multispace0, tag("head")),
//             preceded(multispace1, parse_num),
//             preceded(multispace0, tag(";")),
//         ),
//     )(input)
// }

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
