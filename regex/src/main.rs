
fn main() {
    parse_range("hello".to_string());
}

struct token {
    tag: String,
    quantifier: String
}

fn get_last(v: Vec<char>) {
    v.last().unwrap();
}

fn parse_range(sub_string: String) {
    let mut  _v: Vec<token> = Vec::new();

    for i in sub_string.chars() {   
        match i {
           '.' => _v.push(token {tag: "wildcard".to_string(), quantifier: "exactlyOne".to_string()}),
           _ => println!("{}", "default case")
        }
    }

}
