# rcs-parser
Parsing RCS ([Revision Control System](https://www.gnu.org/software/rcs/)) files.

[Grammar](https://www.gnu.org/software/rcs/manual/html_node/comma_002dv-grammar.html#comma_002dv-grammar) of RCS files is quite simple, so this parser was easily implemented with the [Nom](https://github.com/Geal/nom) parser combinator library. 

NOTE:
This project is under heavy development. Current status is **30%**.

Currently revision numbers, delta texts, delta and strings can be parsed with this library. RCS admin section is missing.

TODO: 
- instring parser
- admin parser
- adding newphrase (RCS V5.8) parser and newphrase to admin, delta, deltatext
- generic parser functions for admin and delta

