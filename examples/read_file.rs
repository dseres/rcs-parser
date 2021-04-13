use rcs_parser::parse_rcs;

fn main() {
    let contents = std::fs::read_to_string("examples/text1.txt,v").unwrap();
    let (_input, rcs) = parse_rcs(contents.as_str()).unwrap();
    println!("{:?}", rcs);
}
