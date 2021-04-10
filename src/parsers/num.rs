#![allow(dead_code)]

use crate::Num;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map,
    error::{context, VerboseError},
    multi::separated_list1,
    IResult,
};

/// Parsing a revision number.
///
/// Grammar of num is:
/// > num       ::=  { digit | "." }+
///
/// Example:
/// ```ignore
/// use rcs_parser::{Num,num};
/// use nom::{
///     error::{ErrorKind, VerboseError, VerboseErrorKind},
///     Err,
/// };
///
/// # fn not_needed(){
/// assert_eq!(Ok(("", num![1, 1])), parse_num("1.1"));
///
/// assert_eq!(Ok(("abc", num![1, 2, 4])), parse_num("1.2.4abc"));
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
/// # }
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
    use crate::{num, Num};
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err,
    };

    #[test]
    fn parse_num() {
        assert_eq!(Ok(("", num![1])), super::parse_num("1"));
        assert_eq!(Ok(("", num![1, 1])), super::parse_num("1.1"));
        assert_eq!(Ok(("", num![1, 1, 1])), super::parse_num("1.1.1"));
        assert_eq!(
            Ok(("w", num![134, 1, 4, 2])),
            super::parse_num("134.1.4.2w")
        );
        assert_eq!(Ok(("a.1.4.2w", num![134])), super::parse_num("134a.1.4.2w"));
        assert_eq!(
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("  1", VerboseErrorKind::Nom(ErrorKind::Digit)),
                    ("  1", VerboseErrorKind::Context("Num"))
                ]
            })),
            super::parse_num("  1")
        );
    }
}
