pub mod ast;
pub mod lexer;
pub mod token;

pub struct Parser {
	pub current: token::Token,
	pub lexer: lexer::Lexer,
	pub peeked: Option<token::Token>,
}

impl Parser {
	pub fn new(input: &str) -> Parser {
		let l = lexer::Lexer::new(input);
		let p = Parser {
			current: token::EOF,
			peeked: None,
			lexer: l
		};
//		p.next_token();
		p
	}

	pub fn parse(&mut self) -> Box<ast::Node> {
		self.expr(1)
	}

	pub fn expr(&mut self, prec: uint) -> Box<ast::Node> {
		let mut lhs = self.atom();
		//println!("expr {}", self.current);		
		let mut rhs;
		loop {
			let curr = self.peek_token();
			//println!("{}", curr);
			if token::is_eof(curr) {
				//println!("breaking out of expr loop");
				break;
			}
			let info_tuple = curr.info();
			if info_tuple.is_none() {
				break;
			}
			let (op_prec, op_assoc) = info_tuple.unwrap();
			if op_prec < prec {
				break;
			}
			self.next_token();
			match op_assoc {
				0 => {
					rhs = self.expr(op_prec + 1);
				}
				_  => {
					rhs = self.expr(op_prec)		
				}
			}
			lhs = self.op(curr, lhs, rhs);

		}
		//println!("expr ending");
		return lhs;
    	//box ast::Add {
        //	left: box ast::Num { num: 1.0 } as Box<ast::Node>,
        //	right: box ast::Num { num: 2.0 } as Box<ast::Node>,
    	//} as Box<ast::Node> 
	}

	pub fn atom(&mut self) -> Box<ast::Node> {
		
		match self.peek_token() {
			token::EOF => {return box ast::Num {num: 0f64} as Box<ast::Node>;}
			token::LPAREN => {
				//println!("looking for subexpr");
				self.expect('(');
				let e = self.expr(1);
				self.expect(')');
				//println!("found subexpr");
				e
			}
			token::NUMBER(val) => {
				//println!("found number {}", val);
				self.next_token();
				box ast::Num { num: val } as Box<ast::Node> 
			}
			a => {
				fail!("unrecognized atom: {}", a);
				box ast::Num { num: 0f64} as Box<ast::Node>
			}
		}
	}

	pub fn op(&self, op: token::Token, lhs: Box<ast::Node>, rhs: Box<ast::Node>)
			-> Box<ast::Node> {
		match op {
			token::ADD => {
				box ast::Add {
					left: lhs, 
					right: rhs
				} as Box<ast::Node>
			}
			token::SUB => {
				box ast::Sub {
					left: lhs, 
					right: rhs
				} as Box<ast::Node>

			}
			token::MUL => {
				box ast::Mul {
					left: lhs, 
					right: rhs
				} as Box<ast::Node>
			}
			token::DIV => {
				box ast::Div {
					left: lhs, 
					right: rhs
				} as Box<ast::Node>
			}
			token::CARET => {
				box ast::Pow {
					base: lhs,
					exponent: rhs
				} as Box<ast::Node>
			}
			o => {
				fail!("unrecognized op: {}", o);
				box ast::Num {num: 0f64} as Box<ast::Node>
			}
		}
	}
}

impl Parser {
	pub fn expect(&mut self, tok: char) {
		self.next_token();
		if self.current.to_char() != tok {
			fail!("expected {} but found {}", tok, self.current.to_char());
		}
	}
	pub fn peek_token(&mut self) -> token::Token{
		if self.peeked.is_none() {
			self.peeked = Some(self.lexer.next_token());
		}
		return self.peeked.unwrap();
	}
	pub fn next_token(&mut self) {
		match self.peeked {
			Some(pk) => {
				self.current = pk;
			}
			None => {
				self.current = self.lexer.next_token();
			}
		}
		self.peeked = None;
	}
}
