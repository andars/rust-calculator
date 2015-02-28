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
            l.curr = src.char_at(0);
        }
        l
    }
    pub fn next_token(&mut self) -> token::Token {
        //println!("eof status: {}", self.eof);
        if self.eof {
//            println!("EOF!!!!!");
            return EOF;
        }
        self.consume_whitespace();
        match self.curr {
            '(' => {self.bump(); LPAREN}
            ')' => {self.bump(); RPAREN}
            c if c.is_digit(10) => {
                let start = self.pos;
                let mut end = start + 1;
                self.bump();
                while (self.curr.is_digit(10) || self.curr == '.') && !self.eof{
                    self.bump();
                    end += 1;
                }
                NUMBER(self.src[start..end].parse::<f64>().unwrap())
            }
            c if c.is_alphabetic() => {
                let start = self.pos;
                let mut end = start + 1;
                self.bump();
                while self.curr.is_alphabetic() && !self.eof {
                    self.bump();
                    end += 1;
                }
                SYMBOL(self.src[start..end].to_string())
            }
            '+' => {self.bump(); ADD}
            '-' => {self.bump(); SUB}
            '*' => {self.bump(); MUL}
            '/' => {self.bump(); DIV}
            '^' => {self.bump(); CARET}
            c => { panic!("unexpected token {} at postion {}", c, self.pos); }
        }
    }
    pub fn bump(&mut self) {
        //println!("bumping to pos: {} of {}", self.pos+1, self.src.as_slice().len());
        self.pos += 1;
        if self.pos >= self.src.len() {
            self.eof = true;
            return;
        }
        self.curr = self.src.char_at(self.pos);
        
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
