
fn main() {
    parse_range(".?");
}


struct Token{
    tag: String,
    quantifier: String
}


fn parse_range(sub_string: &str) {
    let mut  _v: Vec<Vec<Token>> = Vec::new();
    let mut default: Token = Token{tag: "NULL".to_string(), quantifier: "NULL".to_string()};

    for i in sub_string.chars() {   
        match i {
           '.' => {
                _v.last_mut().unwrap().push(Token {tag: "wildcard".to_string(), quantifier: "exactlyOne".to_string()});
           },
           '?' => {
                let _last: &mut Token = _v.last_mut().unwrap().last_mut().unwrap_or(&mut default);
                if _last.quantifier != "exactlyOne" {
                    println!("Error!!!");
                    return;
                }

                _last.quantifier = "exactlyOne".to_string();
           },
           '*' => {
                let _last: &mut Token = _v.last_mut().unwrap().last_mut().unwrap_or(&mut default);
                if _last.quantifier != "exactlyOne" {
                    println!("Error!!!");
                    return;
                }

                _last.quantifier = "zeroOrMore".to_string();
           },
           '+' => {
                let _last: &Token = _v.last().unwrap().last().unwrap_or(&default);
                if _last.quantifier != "exactlyOne" {
                    println!("Error!!!");
                    return;
                }

                _v.last_mut().unwrap().push(Token {tag: _last.tag.clone(), quantifier: "zeroOrMore".to_string()});
           },
           _ => println!("{}", "default case")
        }
    }

}
