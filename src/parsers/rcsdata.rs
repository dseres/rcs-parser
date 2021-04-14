#![allow(dead_code)]

use crate::{parsers::*, *};
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace0, multispace1},
    error::{context, VerboseError},
    multi::many0,
    sequence::preceded,
    IResult,
};
use std::collections::BTreeMap;

pub static CONTEXT: &str = "RCS";

pub fn parse_rcs(input: &str) -> IResult<&str, RcsData, VerboseError<&str>> {
    let (input, mut rcsdata) = context(CONTEXT, parse_admin)(input)?;
    let (input, deltas) = context(CONTEXT, parse_deltas)(input)?;
    let (input, desc) = context(CONTEXT, parse_desc)(input)?;
    let (input, deltatexts) = context(CONTEXT, parse_deltatexts)(input)?;
    let (input, _) = context(CONTEXT, line_ending)(input)?;
    rcsdata.desc = desc;
    rcsdata.deltas = build_deltas(deltas, deltatexts);
    Ok((input, rcsdata))
}

fn parse_deltas(input: &str) -> IResult<&str, Vec<Delta>, VerboseError<&str>> {
    context("deltas", many0(parse_delta))(input)
}

fn parse_desc(input: &str) -> IResult<&str, String, VerboseError<&str>> {
    context(
        "desc",
        preceded(
            preceded(multispace1, tag("desc")),
            preceded(multispace1, parse_string),
        ),
    )(input)
}

fn parse_deltatexts(input: &str) -> IResult<&str, Vec<DeltaText>, VerboseError<&str>> {
    let (input, delta_head) =
        context("deltatexts", preceded(multispace0, parse_deltatext_head))(input)?;
    let (input, mut deltatexts) =
        context("deltatexts", many0(preceded(multispace0, parse_deltatext)))(input)?;
    deltatexts.insert(0, delta_head);
    Ok((input, deltatexts))
}

fn build_deltas(mut deltas: Vec<Delta>, mut texts: Vec<DeltaText>) -> BTreeMap<Num, Delta> {
    let mut dtree = BTreeMap::new();
    for d in deltas.drain(..) {
        dtree.insert(d.num.clone(), d);
    }
    for t in texts.drain(..) {
        let mut d = dtree.get_mut(&(t.num)).unwrap();
        d.log = t.log;
        d.text = t.text;
    }
    dtree
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn parse_deltas() {
        let input = r#"


2.1
date	2021.04.10.09.38.42;	author dseres;	state Production;
branches;
next	1.2;

1.2
date	2021.03.25.10.16.43;	author dseres;	state beta;
branches
	1.2.1.1
	1.2.2.1;
next	1.1;"#;
        let (input, deltas) = super::parse_deltas(input).unwrap();
        assert_eq!("", input);
        assert_eq!(2, deltas.len());
    }

    #[test]
    fn parse_desc() {
        let input = r#"

            desc
            @initial commit
            text from lao
            @"#;
        let result = String::from(
            r#"initial commit
            text from lao
            "#,
        );
        assert_eq!(Ok(("", result)), super::parse_desc(input));
    }

    #[test]
    fn parse_deltatexts() {
        let input = r#"


2.1
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
@


1.2
log
@Tzu has given some new idea. 

Maybe it is a @@useful@@ idea.
@
text
@d1 2
d4 1
a4 2
The named is the mother of all things.

a11 3
They both may be called deep and profound.
Deeper and more profound,
The door of all subtleties!
@"#;
        let (input, deltatexts) = super::parse_deltatexts(input).unwrap();
        assert_eq!("", input);
        assert_eq!(2, deltatexts.len());
    }

    #[test]
    fn parse_rcs() {
        let contents = std::fs::read_to_string("examples/text1.txt,v").unwrap();
        let (input, rcs) = super::parse_rcs(contents.as_str()).unwrap();
        assert_eq!(input, "");
        assert_eq!(rcs.head, num![2, 1]);
        assert_eq!(rcs.deltas.len(), 7);
        assert_eq!(rcs.desc, "initial commit\ntext from lao\n");
    }
}
