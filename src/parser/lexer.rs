use std::char;
use std::fmt;
use parser::token;

pub struct Lexer {
	pub curr:  char,
	pub pos: uint,
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
//			println!("EOF!!!!!");
			return token::EOF;
		}
		self.consume_whitespace();
		match self.curr {
			'(' => {self.bump(); token::LPAREN}
			')' => {self.bump(); token::RPAREN}
			c if char::is_digit(c) => {
				let start = self.pos;
				let mut end = start + 1;
				self.bump();
				while (char::is_digit(self.curr) || self.curr == '.') && !self.eof{
					self.bump();
					end += 1;
				}
				token::NUMBER(from_str(self.src.as_slice().slice(start,end)).unwrap())
			}
			c if char::is_alphabetic(c) => {
				let start = self.pos;
				let mut end = start + 1;
				self.bump();
				while char::is_alphabetic(self.curr) && !self.eof {
					self.bump();
					end += 1;
				}
				token::SYMBOL(self.src.as_slice().slice(start,end).to_string())
			}
			'+' => {self.bump(); token::ADD}
			'-' => {self.bump(); token::SUB}
			'*' => {self.bump(); token::MUL}
			'/' => {self.bump(); token::DIV}
			'^' => {self.bump(); token::CARET}
			c => { fail!("unexpected token {} at postion {}", c, self.pos); }
		}
	}
	pub fn bump(&mut self) {
		//println!("bumping to pos: {} of {}", self.pos+1, self.src.as_slice().len());
		self.pos += 1;
		if self.pos >= self.src.as_slice().len() {
			self.eof = true;
			return;
		}
		self.curr = self.src.as_slice().char_at(self.pos);
		
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


impl fmt::Show for Lexer {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.src)
	}
}
