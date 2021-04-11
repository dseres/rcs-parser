#![allow(dead_code)]

use crate::{parsers::*, *};
use nom::{
    character::complete::{multispace0},
    combinator::{map, opt},
    error::{context, VerboseError},
    sequence::preceded,
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
    let (input, date) = parse_value(CONTEXT, "date", parse_num)(input)?;
    let (input, author) = parse_value(CONTEXT, "author", map(parse_id, String::from))(input)?;
    let (input, state) = parse_value_opt(CONTEXT, "state", map(parse_id, String::from))(input)?;
    let (input, branches) = parse_value_many0(CONTEXT, "branches", parse_num)(input)?;
    let (input, next) = parse_value(CONTEXT, "next", opt(parse_num))(input)?;
    let (input, commitid) = parse_value_all_opt(CONTEXT, "commitid", map(parse_sym, String::from))(input)?;
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

#[cfg(test)]
mod test {
    use crate::{num, Delta, Num};


    /*
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
        assert_eq!(Ok(("", Some(num![1, 1]))), super::parse_next("\nnext 1.1;"));
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
    */
    #[test]
    fn parse_delta() {
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
            branches: vec![num![1, 2, 1, 1], num![1, 2, 2, 1]],
            next: Some(num![1, 1]),
            commitid: None,
        };
        assert_eq!(Ok(("", delta)), super::parse_delta(delta_str));
    }
}
