use std::fmt;
use parser::token;
use parser::token::Token::*;

pub struct Lexer {
    pub curr:  char,
    pub pos: usize,
    pub src: String,
    pub eof: bool
}

impl Lexer {
    pub fn new(src: &str) -> Lexer {
        let mut l = Lexer {
            curr: '\0',
            pos: 0,
            src: src.to_string(),
            eof: false
        };
        if l.pos >= src.len() {
            l.eof = true;
        } else {
            l.curr = src.chars().nth(0).unwrap();
        }
        l
    }
    pub fn next_token(&mut self) -> Result<token::Token, String> {
        if self.eof {
            return Ok(EOF);
        }
        self.consume_whitespace();
        match self.curr {
            '(' => {self.bump(); Ok(LPAREN)}
            ')' => {self.bump(); Ok(RPAREN)}
            c if c.is_digit(10) => {
                let start = self.pos;
                let mut end = start + 1;
                self.bump();
                while (self.curr.is_digit(10) || self.curr == '.') && !self.eof{
                    self.bump();
                    end += 1;
                }
                Ok(NUMBER(self.src[start..end].parse::<f64>().unwrap()))
            }
            c if c.is_alphabetic() => {
                let start = self.pos;
                let mut end = start + 1;
                self.bump();
                while self.curr.is_alphabetic() && !self.eof {
                    self.bump();
                    end += 1;
                }
                Ok(SYMBOL(self.src[start..end].to_string()))
            }
            '+' => {self.bump(); Ok(ADD)}
            '-' => {self.bump(); Ok(SUB)}
            '*' => {self.bump(); Ok(MUL)}
            '/' => {self.bump(); Ok(DIV)}
            '^' => {self.bump(); Ok(CARET)}
            '=' => {self.bump(); Ok(EQUALS)}
            c => { Err(format!("unexpected token {} at position {}", c, self.pos)) }
        }
    }
    pub fn bump(&mut self) {
        self.pos += 1;
        if self.pos >= self.src.len() {
            self.eof = true;
            return;
        }
        self.curr = self.src.chars().nth(self.pos).unwrap();
    }

    pub fn consume_whitespace(&mut self) {
        while is_whitespace(self.curr) {
            self.bump();
        }
    }
}
pub fn is_whitespace(c: char) -> bool {
    match c {
        ' ' | '\n' | '\t' => true,
        _ => false
    }
}


impl fmt::Display for Lexer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.src)
    }
}
