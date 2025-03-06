
fn main() {
    parse_range("hello");
}


#[derive(Copy, Clone)]
struct Token<'lifetime> {
    tag: &'lifetime str,
    quantifier: &'lifetime str
}

fn get_last(v: Vec<Token>) {
    v.last().unwrap();
}

fn parse_range(sub_string: &str) {
    let mut  _v: Vec<Token> = Vec::new();

    for i in sub_string.chars() {   
        match i {
           '.' => {
                _v.push(Token {tag: "wildcard", quantifier: "exactlyOne"})
           },
           '?' => {
                let last = get_last(_v.clone());
           },
           _ => println!("{}", "default case")
        }
    }

}
