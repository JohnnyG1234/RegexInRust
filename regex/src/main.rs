
fn main() {
    parse_range("(.?");
}


#[derive(Clone)]
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
                let _last: Token = _v.last().unwrap().last().unwrap_or(&default).clone();
                if _last.quantifier != "exactlyOne" {
                    println!("Error!!!");
                    return;
                }

                _v.last_mut().unwrap().push(Token {tag: _last.tag.clone(), quantifier: "zeroOrMore".to_string()});
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
                _v.last_mut().unwrap().push(Token {tag: "groupElement".to_string(), quantifier: "exactlyOne".to_string()});
           },
           '\\' => {
                //if 
                /*
                    The iterator is moved into the for loop. You cannot manually manipulate an iterator inside a for loop. However, the for loop can be replaced by while let:
                    while let Some(c) = iter.next() {
                    let current: char = c;
                    let next: char = *iter.peek().unwrap_or(&'∅');
                    }
                    https://stackoverflow.com/questions/72787359/peek-iterator-inside-for-loop
                 */
           }
           _ => println!("{}", "default case")
        }
    }

}
