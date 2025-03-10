
fn main() {
    let s: Vec<Token> = parse_range("a?(b.c*c)+d");

    let mut iter = s.iter();
    while let Some(t) = iter.next(){
        println!(" ");
        println!("{}", t.value.to_string());
        println!("{}", t.tag.to_string());
        println!("{}", t.quantifier.to_string());
        println!(" ");
    }
}


#[derive(Clone)]
struct Token{
    tag: String,
    quantifier: String,
    value: char
}


fn parse_range(sub_string: &str) -> Vec<Token>{
    let mut  _v: Vec<Vec<Token>> = Vec::new();
    _v.push(vec![]);

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
                    return _v.first().unwrap().clone();
                }

                _last.quantifier = "exactlyOne".to_string();
           },
           '*' => {
                let _last: &mut Token = _v.last_mut().unwrap().last_mut().unwrap_or(&mut default);
                if _last.quantifier != "exactlyOne" {
                    println!("Error!!!");
                    return _v.first().unwrap().clone();
                }

                _last.quantifier = "zeroOrMore".to_string();
           },
           '+' => {
                let _last: Token = _v.last().unwrap().last().unwrap_or(&default).clone();
                if _last.quantifier != "exactlyOne" {
                    println!("Error!!!");
                    return _v.first().unwrap().clone();
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
                    return _v.first().unwrap().clone();
                }
                _v.pop();
                _v.last_mut().unwrap().push(Token {tag: "groupElement".to_string(), quantifier: "exactlyOne".to_string(), value: ' '});
           },
           '\\' => {
                let next: char = *iter.peek().unwrap_or(&'∅');
                if next == '∅' {
                    println!("No character after backslash");
                    return _v.first().unwrap().clone();
                }
                _v.last_mut().unwrap().push(Token {tag: "element".to_string(), quantifier: "exactlyOne".to_string(), value: next});
                iter.nth(0);
           }
           _ => {
                _v.last_mut().unwrap().push(Token {tag: "element".to_string(), quantifier: "exactlyOne".to_string(), value: c});
           }
        }
    }
    if _v.len() != 1 {
        println!("ERROR!!!");
        return _v.first().unwrap().clone();
    }

    return _v.first().unwrap().clone();
}
