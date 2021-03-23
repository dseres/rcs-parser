extern crate nom;
extern crate rcs_parser;

use nom::{error::convert_error, Err};
use rcs_parser::*;

#[derive(Debug, Clone)]
struct Teszt{
    errors: Vec<String>,
}
#[cfg(not(tarpaulin_include))]
fn main() {
    let data = "@Helló világ!";
    println!("{}", data);
    let result = parse_string(data);
    println!("{:?}", &result);
    match result {
        Err(Err::Error(e)) => println!("Error in input:\n{}", convert_error(data, e)),
        _ => (),
    }

    let t = Teszt{errors: vec!["aaa".to_string(),"bbb".to_string()]};
    println!("{:?}", &t);

}
