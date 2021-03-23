#![allow(dead_code)]

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map,
    error::{context, VerboseError},
    multi::separated_list1,
    IResult,
};

/// Num stores an RCS revision number as vector of unsigned integers.
///
/// E.g.: 1.2.3.4 will be represented as :
/// ```rust
/// # use rcs_parser::Num;
/// # fn not_needed()-> Num{
/// Num{numbers:vec![1,2,3,4]}
/// # }
/// ```
#[derive(Debug, PartialEq)]
pub struct Num {
    ///The numbers of a revision number
    pub numbers: Vec<u32>,
}

impl Num {
    pub fn new(numbers: Vec<u32>) -> Num {
        Num { numbers }
    }
}

/// Parsing a revision number.
///
/// Grammar of num is:
/// > num       ::=  { digit | "." }+
///
/// Example:
/// ```rust
/// use rcs_parser::{Num,parse_num};
/// use nom::{
///     error::{ErrorKind, VerboseError, VerboseErrorKind},
///     Err,
/// };
///
/// assert_eq!(Ok(("", Num::new(vec![1, 1]))), parse_num("1.1"));
///
/// assert_eq!(Ok(("abc", Num::new(vec![1, 2, 4]))), parse_num("1.2.4abc"));
///
/// assert_eq!(
///     Err(Err::Error(VerboseError {
///         errors: vec![
///             ("", VerboseErrorKind::Nom(ErrorKind::Digit)),
///             ("", VerboseErrorKind::Context("Num"))
///         ]
///     })),
///     parse_num("")
/// );
/// 
/// assert_eq!(
///     Err(Err::Error(VerboseError {
///         errors: vec![
///             ("not_number", VerboseErrorKind::Nom(ErrorKind::Digit)),
///             ("not_number", VerboseErrorKind::Context("Num"))
///         ]
///     })),
///     parse_num("not_number")
/// );
/// ```
pub fn parse_num(input: &str) -> IResult<&str, Num, VerboseError<&str>> {
    context(
        "Num",
        map(
            separated_list1(
                tag("."),
                map(digit1, |d| u32::from_str_radix(d, 10).unwrap()),
            ),
            |numbers| Num { numbers },
        ),
    )(input)
}

#[cfg(test)]
mod test {
    use crate::parsers::num;
    use crate::parsers::num::Num;
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err,
    };

    #[test]
    fn parse_num() {
        assert_eq!(Ok(("", Num::new(vec![1]))), num::parse_num("1"));
        assert_eq!(Ok(("", Num::new(vec![1, 1]))), num::parse_num("1.1"));
        assert_eq!(Ok(("", Num::new(vec![1, 1, 1]))), num::parse_num("1.1.1"));
        assert_eq!(
            Ok(("w", Num::new(vec![134, 1, 4, 2]))),
            num::parse_num("134.1.4.2w")
        );
        assert_eq!(
            Ok(("a.1.4.2w", Num::new(vec![134]))),
            num::parse_num("134a.1.4.2w")
        );
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("  1", VerboseErrorKind::Nom(ErrorKind::Digit)),
                    ("  1", VerboseErrorKind::Context("Num"))
                ]
            })),
            num::parse_num("  1")
        );
    }
}
