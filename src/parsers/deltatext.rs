#![allow(dead_code)]

// Parsing deltatext
//
// Grammar of deltatext is:
// > deltatext ::=  num
// >                "log"   string
// >                "text"  string
// Example:
// ```rust
// use rcs_parser::{Num,parse_num};
// use nom::{
//     error::{ErrorKind, VerboseError, VerboseErrorKind},
//     Err,
// };
//
// assert_eq!(Ok(("", Num::new(vec![1, 1]))), parse_num("1.1"));
//
// assert_eq!(Ok(("abc", Num::new(vec![1, 2, 4]))), parse_num("1.2.4abc"));
//
// assert_eq!(
//     Err(Err::Error(VerboseError {
//         errors: vec![
//             ("", VerboseErrorKind::Nom(ErrorKind::Digit)),
//             ("", VerboseErrorKind::Context("Num"))
//         ]
//     })),
//     parse_num("")
// );
//
// assert_eq!(
//     Err(Err::Error(VerboseError {
//         errors: vec![
//             ("not_number", VerboseErrorKind::Nom(ErrorKind::Digit)),
//             ("not_number", VerboseErrorKind::Context("Num"))
//         ]
//     })),
//     parse_num("not_number")
// );
// ```
/*pub fn parse_deltatext(input: &str) -> IResult<&str, Deltatext, VerboseError<&str>> {
    context(
        "DeltaText",
        map(
            tuple( parse_num, preceded(multispace0, parse_string), preceded(multispace0, parse_string) ),
            |(num,log,delta)| Deltatext{num,log,delta} )
        )(input)
}*/

/*
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
*/
