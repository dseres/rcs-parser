#![allow(dead_code)]

use crate::*;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    error::{context, VerboseError},
    multi::many0,
    sequence::{delimited, preceded},
    IResult,
};

pub static CONTEXT: &str = "Delta";

/// Parsing delta part of comma-v files.
///
/// Grammar of delta is the following:
/// > delta     ::=  num
/// >   "date"       num ";"
/// >   "author"     id ";"
/// >   "state"      {id} ";"
/// >   "branches"   {num}* ";"
/// >   "next"       {num} ";"
/// >   { "commitid" sym ";" }
///
pub fn parse_delta(input: &str) -> IResult<&str, Delta, VerboseError<&str>> {
    let (input, num) = context(CONTEXT, preceded(multispace0, parse_num))(input)?;
    let (input, date) = parse_date(input)?;
    let (input, author) = parse_author(input)?;
    let (input, state) = parse_state(input)?;
    let (input, branches) = parse_branches(input)?;
    let (input, next) = parse_next(input)?;
    let (input, commitid) = parse_commitid(input)?;
    Ok((
        input,
        Delta {
            num,
            date,
            author,
            state,
            branches,
            next,
            commitid,
        },
    ))
}

fn parse_date(input: &str) -> IResult<&str, Num, VerboseError<&str>> {
    context(
        CONTEXT,
        delimited(
            preceded(multispace1, tag("date")),
            preceded(multispace1, parse_num),
            preceded(multispace0, tag(";")),
        ),
    )(input)
}

fn parse_author(input: &str) -> IResult<&str, String, VerboseError<&str>> {
    context(
        CONTEXT,
        delimited(
            preceded(multispace1, tag("author")),
            preceded(multispace1, map(parse_id, |s| String::from(s))),
            preceded(multispace0, tag(";")),
        ),
    )(input)
}

fn parse_state(input: &str) -> IResult<&str, Option<String>, VerboseError<&str>> {
    context(
        CONTEXT,
        delimited(
            preceded(multispace1, tag("state")),
            opt(preceded(multispace1, map(parse_id, |s| String::from(s)))),
            preceded(multispace0, tag(";")),
        ),
    )(input)
}

fn parse_branches(input: &str) -> IResult<&str, Vec<Num>, VerboseError<&str>> {
    context(
        CONTEXT,
        delimited(
            preceded(multispace1, tag("branches")),
            many0(preceded(multispace1, parse_num)),
            preceded(multispace0, tag(";")),
        ),
    )(input)
}

fn parse_next(input: &str) -> IResult<&str, Option<Num>, VerboseError<&str>> {
    context(
        CONTEXT,
        delimited(
            preceded(multispace1, tag("next")),
            opt(preceded(multispace1, parse_num)),
            preceded(multispace0, tag(";")),
        ),
    )(input)
}

fn parse_commitid(input: &str) -> IResult<&str, Option<String>, VerboseError<&str>> {
    context(
        CONTEXT,
        opt(delimited(
            preceded(multispace1, tag("commitid")),
            preceded(multispace1, map(parse_sym, |s| String::from(s))),
            preceded(multispace0, tag(";")),
        )),
    )(input)
}

#[cfg(test)]
mod test {
    use crate::{Delta, Num, num};
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err,
    };

    #[test]
    fn parse_date() {
        assert_eq!(
            Ok(("", num![2021, 04, 07, 12, 00, 00])),
            super::parse_date("\ndate 2021.04.07.12.00.00;")
        );
        assert_eq!(
            Ok(("", num![2021, 04, 07, 12, 00, 00])),
            super::parse_date(" date \t\r\n2021.04.07.12.00.00 ;")
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    (
                        "2021.04.07.12.00.00;",
                        VerboseErrorKind::Nom(ErrorKind::Tag)
                    ),
                    (
                        " 2021.04.07.12.00.00;",
                        VerboseErrorKind::Context(super::CONTEXT)
                    )
                ]
            })),
            super::parse_date(" 2021.04.07.12.00.00;")
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    (
                        " date 2021.04.07.12.00.00",
                        VerboseErrorKind::Context(super::CONTEXT)
                    )
                ]
            })),
            super::parse_date(" date 2021.04.07.12.00.00")
        );
    }

    #[test]
    fn parse_author() {
        assert_eq!(
            Ok(("", String::from("dseres"))),
            super::parse_author("\nauthor dseres;")
        );
    }

    #[test]
    fn parse_state() {
        assert_eq!(
            Ok(("", Some(String::from("testing")))),
            super::parse_state("\nstate testing;")
        );
        assert_eq!(Ok(("", None)), super::parse_state("\nstate;"));
    }

    #[test]
    fn parse_branches() {
        fn parse_state() {
            assert_eq!(
                Ok(("", vec![num![1, 2, 13], num![1, 2, 14]])),
                super::parse_branches("\nbranches 1.2.13, 1.2.14;")
            );
            assert_eq!(
                Ok(("", vec![num![1, 2, 13]])),
                super::parse_branches("\nbranches 1.2.13;")
            );
            assert_eq!(Ok(("", vec![])), super::parse_branches("\nbranches;"));
        }
    }

    #[test]
    fn parse_next() {
        assert_eq!(
            Ok(("", Some(num![1, 1]))),
            super::parse_next("\nnext 1.1;")
        );
        assert_eq!(Ok(("", None)), super::parse_next("\nnext;"));
    }

    #[test]
    fn parse_commitid() {
        assert_eq!(
            Ok(("", Some(String::from("abc")))),
            super::parse_commitid("\n commitid abc;")
        );
        assert_eq!(Ok(("\n", None)), super::parse_commitid("\n"));
    }

    #[test]
    fn name() {
        let delta_str = r#"1.2
            date    2021.03.25.10.16.43;    author dseres;  state beta;
            branches
                    1.2.1.1
                    1.2.2.1;
            next    1.1;"#;
        let delta = Delta {
            num: num![1, 2],
            date: num![2021, 03, 25, 10, 16, 43],
            author: String::from("dseres"),
            state: Some(String::from("beta")),
            branches: vec![ num![1, 2, 1, 1], num![1, 2, 2, 1]],
            next: Some(num![1, 1]),
            commitid: None,
        };
        assert_eq!(Ok(("",delta)), super::parse_delta(delta_str));
    }
}
