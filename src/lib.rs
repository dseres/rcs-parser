//#![warn(missing_docs)]

//! # rcs-parser
//! Parsing RCS ([Revision Control System](https://www.gnu.org/software/rcs/)) files.
//!
//! [Grammar](https://www.gnu.org/software/rcs/manual/html_node/comma_002dv-grammar.html#comma_002dv-grammar) of RCS files is quite simple, so this parser was easily implemented with the [Nom](https://github.com/Geal/nom) parser combinator library.
//!
//! NOTE:
//! This project is under heavy development. Current status is **30%**.
//!
//! Currently revision numbers, delta texts, delta and strings can be parsed with this library. RCS admin section is missing.

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod parsers;

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

#[derive(Debug, PartialEq, Clone)]
pub struct Admin {
    pub head: Num,
    pub branch: Option<Num>,
    pub access: Vec<String>,
    pub symbols: Vec<(String, Num)>,
    pub locks: Vec<(String, Num)>,
    pub strict: bool,
    pub integrity: Option<String>,
    pub comment: Option<String>,
    pub expand: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn num() {
        assert_eq!(Num { numbers: vec![100] }, num!(100));
        assert_eq!(
            Num {
                numbers: vec![1, 1]
            },
            num!(1, 1)
        );
        assert_eq!(
            Num {
                numbers: vec![1, 1, 2, 1]
            },
            num!(1, 1, 2, 1)
        );
    }
}
