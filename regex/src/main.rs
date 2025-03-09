
fn main() {
    parse_range("(.?");
}


#[derive(Clone)]
struct Token{
    tag: String,
    quantifier: String,
    value: char
}


fn parse_range(sub_string: &str) {
    let mut  _v: Vec<Vec<Token>> = Vec::new();

    let mut default: Token = Token{tag: "NULL".to_string(), quantifier: "NULL".to_string(), value: ' '};

    let mut iter = sub_string.chars().peekable();
    while let Some(c) = iter.next() {   
        match c {
           '.' => {
                _v.last_mut().unwrap().push(Token {tag: "wildcard".to_string(), quantifier: "exactlyOne".to_string(), value: ' '});
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
                let _last: Token = _v.last().unwrap().last().unwrap_or(&default).clone();
                if _last.quantifier != "exactlyOne" {
                    println!("Error!!!");
                    return;
                }

                _v.last_mut().unwrap().push(Token {tag: _last.tag.clone(), quantifier: "zeroOrMore".to_string(), value: ' '});
           },
           '(' => {
                let new_v: Vec<Token> = vec![];
                _v.push(new_v);
           },
           ')' => {
                if _v.len() <= 1 {
                    println!("No Group to close");
                    return;
                }
                _v.pop();
                _v.last_mut().unwrap().push(Token {tag: "groupElement".to_string(), quantifier: "exactlyOne".to_string(), value: ' '});
           },
           '\\' => {
                let next: char = *iter.peek().unwrap_or(&'∅');
                if next == '∅' {
                    println!("No character after backslash")
                }
                iter.nth(0);
                _v.last_mut().unwrap().push(Token {tag: "element".to_string(), quantifier: "exactlyOne".to_string(), value: next});
           }
           _ => {
                println!("defualt")
           }
        }
    }

}
