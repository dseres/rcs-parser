# rcs-parser
Parsing RCS ([Revision Control System](https://www.gnu.org/software/rcs/)) files.

[Grammar](https://www.gnu.org/software/rcs/manual/html_node/comma_002dv-grammar.html#comma_002dv-grammar) of RCS files is quite simple, so this parser was easily implemented with the [Nom](https://github.com/Geal/nom) parser combinator library. 

NOTE:
This project is under heavy development. Current status is **10%**.

Currently revision numbers, delta texts and strings can be parsed with this library. 