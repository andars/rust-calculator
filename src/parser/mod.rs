pub mod ast;
pub mod lexer;
pub mod token;

pub fn parse(input: &str) -> Box<ast::Node> {
	let mut lexer = lexer::Lexer::new(input);

   	loop {
		let tok = lexer.next_token();
		match tok {
			token::EOF => {break;}
			_ => {}
		}
		print!("{}, ", tok);
   	}
    // just return a static ast for now

    box ast::Add {
        left: box ast::Num { num: 1.0 } as Box<ast::Node>,
        right: box ast::Num { num: 2.0 } as Box<ast::Node>,
    } as Box<ast::Node>
}
