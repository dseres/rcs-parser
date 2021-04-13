# rcs-parser
Parsing RCS ([Revision Control System](https://www.gnu.org/software/rcs/)) files.

[Grammar](https://www.gnu.org/software/rcs/manual/html_node/comma_002dv-grammar.html#comma_002dv-grammar) of RCS files is quite simple, so this parser was easily implemented with [Nom](https://github.com/Geal/nom). Nom is the state of art implementation of parser combinators in Rust. For more details about RCS grammar, see
```shell
man rcsfile
```

Migrating files from RCS to any other version control software is easy, except if you need the history or the logs. Good tools exists for migration sources with history. One of the bests is [cvs2svn](https://github.com/mhagger/cvs2svn) written in python. 

Sometimes an RCS repository contains too much customizations, so there is no a ready-made tool for the migration and a custom tool should be created. This api is written in rust, and will help to parse good old RCS files, which has the *,v* extensions.

You can easily parse comma-v files with this api. 
In the example bellow [examples/text1.txt,v](examples/text1.txt%2Cv) will be parsed.
```rust
use rcs_parser::parse_rcs;

fn main() {
    let contents = std::fs::read_to_string("examples/text1.txt,v").unwrap();
    let (input, rcs) = parse_rcs(contents.as_str()).unwrap();
    println!("{:?}", rcs);
}
```
---

**Note:** Currently the progress of this library is at about 50%. 
You can use the library now, but it has pre-β status. You may consider the api will change later.

---

TODO: 

- adding newphrase (RCS V5.8) parser and newphrase to admin, delta, deltatext.
- implement Display for RcsData
- implement a function to get file content of a revision
- integration test
- write more documentation
- switch jemalloc to optional
- publish crate

