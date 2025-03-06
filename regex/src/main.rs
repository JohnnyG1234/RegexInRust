use std::ptr::null;

fn main() {
    parse_range_quantifier("hello".to_string());
}

fn parse_range_quantifier(sub_string: String) {
    let mut to_parse: String = sub_string;

    println!("{}", to_parse.chars().next().unwrap());
    if to_parse.chars().next().unwrap() != '{'
    {
        println!("{}", "Not Valid Regex!");
    }
    to_parse.pop();
    
}
