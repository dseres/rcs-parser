#![allow(dead_code)]

use crate::{parsers::*, *};
use nom::{
    character::complete::multispace0,
    combinator::opt,
    error::{context, VerboseError},
    sequence::preceded,
    IResult,
};

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
    static CONTEXT: &str = "Delta";
    let (input, num) = context(CONTEXT, preceded(multispace0, parse_num))(input)?;
    let (input, date) = parse_value(CONTEXT, "date", parse_num)(input)?;
    let (input, author) = parse_value(CONTEXT, "author", parse_id)(input)?;
    let (input, state) = parse_value_opt(CONTEXT, "state", parse_id)(input)?;
    let (input, branches) = parse_value_many0(CONTEXT, "branches", parse_num)(input)?;
    let (input, next) = parse_value(CONTEXT, "next", opt(parse_num))(input)?;
    let (input, commitid) = parse_value_all_opt(CONTEXT, "commitid", parse_sym)(input)?;
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
