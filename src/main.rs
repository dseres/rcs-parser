use rcs_parser::*;

#[cfg(not(tarpaulin_include))]
fn main() {
    let num = num!(1, 1);
    println!("{:?}", num);
    let num = num![2];
    println!("{:?}", num);
}
