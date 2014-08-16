#[deriving(Show, PartialEq)]
pub enum Token {
	LPAREN, 
	RPAREN,
	ADD,
	SUB,
	MUL,
	DIV,
	NUMBER(f64),
	EOF
}
