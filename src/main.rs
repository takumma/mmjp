fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Null,
    String(String),
}

#[derive(Debug)]
struct LexerError {
    pub msg: String,
}

impl LexerError {
    fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }

    fn print(&self) {
        println!("{}", self.msg);
    }
}

struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(s: String) -> Self {
        Self {
            input: s.chars().collect(),
            pos: 0,
        }
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            let token = self.next_token().unwrap();
            match token {
                Some(t) => tokens.push(t),
                None => {}
            }
        }
        tokens
    }

    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        if self.input.len() < self.pos {
            return Ok(None);
        }
        match self.input[self.pos] {
            '"' => {
                self.pos += 1;
                self.perse_string()
            }
            c => Err(LexerError::new(&format!("error: unexpected char {}", c))),
        }
    }

    fn perse_string(&mut self) -> Result<Option<Token>, LexerError> {
        let mut s = String::new();
        while let c = self.input[self.pos] {
            match c {
                '"' => {
                    self.pos += 1;
                    break;
                }
                _ => {
                    self.pos += 1;
                    s.push(c);
                }
            }
        }
        Ok(Some(Token::String(s)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = r#""hello""#;
        let tokens = Lexer::new(s.to_string()).tokenize();
        print!("{:?}", tokens);
        assert_eq!(tokens[0], Token::String("hello".to_string()));
    }
}
