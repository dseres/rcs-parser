use nom::{error::convert_error, Err};
use rcs_parser::*;

#[cfg(not(tarpaulin_include))]
fn main() {
    let data = "@Helló világ!";
    println!("{}", data);
    let result = parse_string(data);
    println!("{:?}", &result);
    if let Err(Err::Error(e)) = result {
        println!("Error in input:\n{}", convert_error(data, e));
    }

    println!("{:?}", parse_num("1.2,3.5a"));
    let data = "not_number";
    let result = parse_num(data);
    if let Err(Err::Error(e)) = result {
        println!("Error in input:\n{}", convert_error(data, e));
    }

    let delta_str = r#"1.1
log
@Initial revision
@
text
@a0 2
The Way that can be told of is not the eternal Way;
The name that can be named is not the eternal name.
d2 2
a3 1
The Named is the mother of all things.
d11 3
@"#;
    println!("Parsing the following text:\n{:?}", delta_str);
    if let Ok((_, delta)) = parse_deltatext(delta_str) {
        println!("{:?}", delta);
    };
}
