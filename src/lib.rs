//r#![warn(missing_docs)]

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod parsers;
pub use crate::parsers::deltatext::*;
pub use crate::parsers::diff::{parse_diff_command, parse_diff_line};
pub use crate::parsers::num::parse_num;
pub use crate::parsers::string::parse_string;

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

impl Num {
    pub fn new(numbers: Vec<u32>) -> Num {
        Num { numbers }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DiffCommand {
    Add(u32, Vec<String>),
    Delete(u32, u32),
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeltaText {
    ///The numbers of a revision number
    pub num: Num,
    pub log: String,
    pub diff: Vec<DiffCommand>,
}
