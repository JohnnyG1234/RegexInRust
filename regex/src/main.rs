
fn main() {
    let s: Vec<Token> = parse_range("a.c241");

    for t in s.iter() {
        println!("{}", t.value);
    }

    test(s, "abc".to_string());
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

                _last.quantifier = "zeroOrOne".to_string();
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


fn test(states: Vec<Token>, to_parse: String) -> bool{

    if states.len() <= 0{
        return false
    }


    for current in states.iter().zip(to_parse.chars().into_iter())
    {
        let current_state = current.0;
        let current_char = current.1;
        println!("{}", current_state.value.to_string());
        println!("{}", current_char);
        
        match current_state.quantifier.as_str() {
            "exactlyOne" => {
                if current_state.tag == "wildcard" {
                    continue;
                }
                

                if current_char != current_state.value {
                    println!("Chars don't match when matching for exactly one");
                    return false;
                }
            },
            "zeroOrOne" => {
                // not sure what to do here becuase soemthing does not have to be here?
            },
            "zeroOrMore" => {
                if current_state.tag == "wildcard" {
                    continue;
                }
            }
            _ => {
                println!("Unsuported Element");
                return false;
            }
         }
    }

    println!("Match!!!");
    return true;
}