use std::{iter::Peekable, str::Chars};

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

struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input: s.chars().peekable(),
        }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(token) = self.next_token().unwrap() {
            match token {
                _ => tokens.push(token),
            }
        }

        // loop {
        //     let token = self.next_token().unwrap();
        //     match token {
        //         Some(t) => tokens.push(t),
        //         None => {}
        //     }
        // }
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        match self.input.peek() {
            Some(c) => match c {
                '"' => {
                    self.input.next();
                    self.perse_string_token()
                }
                _ => Err(LexerError::new(&format!("error: unexpected char {}", c))),
            },
            None => Ok(None),
        }
    }

    fn perse_string_token(&mut self) -> Result<Option<Token>, LexerError> {
        let mut s = String::new();
        while let Some(c) = self.input.next() {
            match c {
                '\"' => {
                    break;
                }
                _ => {
                    s.push(c.clone());
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
        let tokens = Lexer::new(s).tokenize().unwrap();
        print!("{:?}", tokens);
        assert_eq!(tokens[0], Token::String("hello".to_string()));
    }
}
