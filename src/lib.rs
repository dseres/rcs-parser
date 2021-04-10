//r#![warn(missing_docs)]

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod parsers;
pub use crate::parsers::deltatext::*;
pub use crate::parsers::diff::{parse_diff_command, parse_diff_line};
pub use crate::parsers::num::parse_num;
pub use crate::parsers::string::parse_string;
pub use crate::parsers::chars::{is_special_chars, is_visible_char,is_idchar, parse_sym, parse_id};

/// Num stores an RCS revision number as vector of unsigned integers.
///
/// E.g.: 1.2.3.4 will be represented as :
/// ```rust
/// # use rcs_parser::Num;
/// # fn not_needed()-> Num{
/// Num{numbers:vec![1,2,3,4]}
/// # }
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Num {
    ///The numbers of a revision number
    pub numbers: Vec<u32>,
}

#[macro_export]
macro_rules! num {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            Num{numbers: temp_vec}
        }
    };
}

///Holds an instruction of diff command
#[derive(Debug, PartialEq, Clone)]
pub enum DiffCommand {
    ///This instruction means add the lines at position
    Add(u32, Vec<String>),
    ///This instruction means delete n (second parameter) lines from position (first parameter).
    Delete(u32, u32),
}

/// holds differences between revisions.
#[derive(Debug, PartialEq, Clone)]
pub struct DeltaText {
    ///The revision number
    pub num: Num,
    ///Commit log
    pub log: String,
    ///Differences between this and its parent revision
    pub diff: Vec<DiffCommand>,
}


#[derive(Debug, PartialEq, Clone)]
pub struct Delta {
    pub num: Num,
    pub date: Num,
    pub author: String,
    pub state: Option<String>,
    pub branches: Vec<Num>,
    pub next: Option<Num>,
    pub commitid: Option<String>,
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn num() {
        assert_eq!(Num{numbers: vec![100]}, num!(100));
        assert_eq!(Num{numbers: vec![1,1]}, num!(1,1));
        assert_eq!(Num{numbers: vec![1,1,2,1]}, num!(1,1,2,1));
    }
}