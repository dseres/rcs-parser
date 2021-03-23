//r#![warn(missing_docs)]

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod parsers;
pub use crate::parsers::num::{Num, parse_num};
pub use crate::parsers::string::parse_string;