# rcs-parser
Parsing RCS ([Revision Control System](https://www.gnu.org/software/rcs/)) files.

[Grammar](https://www.gnu.org/software/rcs/manual/html_node/comma_002dv-grammar.html#comma_002dv-grammar) of RCS files is quite simple, so this parser was easily implemented with [Nom](https://github.com/Geal/nom). Nom is the state of art implementation of parser combinators in Rust. 

You can easily parse comma-v files with this api. 
Example:
```rust
use rcs_parser::parse_rcs;

fn main() {
    let contents = std::fs::read_to_string("examples/text1.txt,v").unwrap();
    let (input, rcs) = parse_rcs(contents.as_str()).unwrap();
    println!("{:?}", rcs);
}
```

TODO: 
- adding newphrase (RCS V5.8) parser and newphrase to admin, delta, deltatext.
- implement Display for RcsData
- integration test
- write more documentation
- switch jemalloc to optional
- publish crate

