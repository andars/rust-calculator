#[deriving(Show, PartialEq, Clone)]
pub enum Token {
	LPAREN, 
	RPAREN,
	ADD,
	SUB,
	MUL,
	DIV,
	CARET,
	NUMBER(f64),
	SYMBOL(String),
	EOF
}

impl Token {
	/* returns (prec, associativity) where 0 is left and 1 is right*/
	pub fn info(&self) -> Option<(uint, uint)> {
		match *self {
			ADD | SUB => Some((10, 0)),
			MUL | DIV => Some((20, 0)),
			CARET => Some((30, 1)),
			_ => { None}
		}
	}
	
	pub fn to_char(&self) -> char {
		match *self {
			LPAREN => '(',
			RPAREN => ')',
			ADD => '+',
			SUB => '-',
			MUL => '*',
			DIV => '/',
			CARET => '^',
			EOF => 'E',
			NUMBER(_) => 'N',
			SYMBOL(_) => 'S',
		}
	}
}

pub fn is_eof(t: Token) -> bool{
	match t {
		EOF => true,
		_ => false
	}
}
