
fn main() {
    parse_range("?");
}


struct Token{
    tag: String,
    quantifier: String
}


fn parse_range(sub_string: &str) {
    let mut  _v: Vec<Token> = Vec::new();
    let default: Token = Token{tag: "NULL".to_string(), quantifier: "NULL".to_string()};

    for i in sub_string.chars() {   
        match i {
           '.' => {
                _v.push(Token {tag: "wildcard".to_string(), quantifier: "exactlyOne".to_string()});
           },
           '?' => {
                let _last: &Token = _v.last().unwrap_or(&default);
                if _last.quantifier != "exactlyOne" {
                    println!("Error!!!")
                }
           },
           _ => println!("{}", "default case")
        }
    }

}
