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
}
