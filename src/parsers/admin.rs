#![allow(dead_code)]

use std::collections::BTreeMap;
use crate::{parsers::*, *};
use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{map, opt},
    error::{context, VerboseError},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

pub static CONTEXT: &str = "Admin";

/// Parsing delta part of comma-v files.
///
/// Grammar of delta is the following:
/// > admin     ::=  "head"         {num} ";"
/// > { "branch"     {num} ";" }
/// > "access"       {id}* ";"
/// > "symbols"      { sym ":" num }* ";"
/// > "locks"        { id ":" num }* ";"
/// > { "strict" ";" }
/// > { "integrity " {intstring} ";" }
/// > { "comment"    {string} ";" }
/// > { "expand"     {string} ";" }
///
///
///
pub fn parse_admin(input: &str) -> IResult<&str, RcsData, VerboseError<&str>> {
    let (input, head) = parse_value(CONTEXT, "head", parse_num)(input)?;
    let (input, branch) = parse_value_all_opt(CONTEXT, "branch", parse_num)(input)?;
    let (input, access) = parse_value_many0(CONTEXT, "access", parse_id)(input)?;
    let (input, symbols) = parse_value_many0(
        CONTEXT,
        "symbols",
        separated_pair(parse_sym, tag(":"), parse_num),
    )(input)?;
    let (input, locks) = parse_value_many0(
        CONTEXT,
        "locks",
        separated_pair(parse_id, tag(":"), parse_num),
    )(input)?;
    let (input, strict) = parse_strict(input)?;
    let (input, integrity) = parse_value_all_opt(CONTEXT, "integrity", parse_intstring)(input)?;
    let (input, comment) = parse_value_all_opt(CONTEXT, "comment", parse_string)(input)?;
    let (input, expand) = parse_value_all_opt(CONTEXT, "expand", parse_string)(input)?;
    Ok((
        input,
        RcsData {
            head,
            branch,
            access,
            symbols,
            locks,
            strict,
            integrity,
            comment,
            expand,
            desc: String::new(),
            deltas: BTreeMap::new(),
        },
    ))
}

fn parse_strict(input: &str) -> IResult<&str, bool, VerboseError<&str>> {
    context(
        CONTEXT,
        map(
            opt(terminated(
                preceded(multispace0, tag("strict")),
                preceded(multispace0, tag(";")),
            )),
            |o| o.is_some(),
        ),
    )(input)
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;
    use crate::*;

    #[test]
    fn parse_strict() {
        assert_eq!(Ok(("", true)), super::parse_strict("strict;"));
        assert_eq!(Ok(("", true)), super::parse_strict(" strict ;"));
        assert_eq!(Ok(("strict", false)), super::parse_strict("strict"));
        assert_eq!(Ok((";", false)), super::parse_strict(";"));
        assert_eq!(Ok(("no", false)), super::parse_strict("no"));
    }

    #[test]
    fn parse_admin() {
        let input = r#"head    2.1;
            access;
            symbols
                    Fix2:1.2.2.3
                    Fix1:1.2.1.1
                    v2_1:2.1
                    v1_1:1.2;
            locks
                    dseres:2.1; strict;
            comment @# @;"#;
        let result = RcsData {
            head: num![2, 1],
            branch: None,
            access: vec![],
            symbols: vec![
                (String::from("Fix2"), num![1, 2, 2, 3]),
                (String::from("Fix1"), num![1, 2, 1, 1]),
                (String::from("v2_1"), num![2, 1]),
                (String::from("v1_1"), num![1, 2]),
            ],
            locks: vec![(String::from("dseres"), num![2, 1])],
            strict: true,
            integrity: None,
            comment: Some(String::from("# ")),
            expand: None,
            desc: String::new(),
            deltas: BTreeMap::new(),
        };
        assert_eq!(Ok(("", result)), super::parse_admin(input));
    }
}
