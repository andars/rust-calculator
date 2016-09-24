pub use self::token::Token::*;

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
            current: EOF,
            peeked: None,
            lexer: l
        };
        p
    }

    pub fn parse(&mut self) -> Result<Box<ast::Node>, String> {
        self.expr(1)
    }

    pub fn expr(&mut self, prec: usize) -> Result<Box<ast::Node>, String> {
        let mut lhs = try!(self.atom());
        let mut rhs;
        loop {
            let curr = try!(self.peek_token());
            //println!("{:?}", curr);
            if token::is_eof(&curr) {
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
            try!(self.next_token());
            match op_assoc {
                0 => {
                    rhs = try!(self.expr(op_prec + 1));
                }
                _  => {
                    rhs = try!(self.expr(op_prec));
                }
            }
            lhs = self.op(curr, lhs, rhs);

        }
        Ok(lhs)
    }

    pub fn atom(&mut self) -> Result<Box<ast::Node>, String> {

        match try!(self.peek_token()) {
            EOF => { Ok(Box::new( ast::Num {num: 0f64})) }
            LPAREN => {
                try!(self.expect('('));
                let e = try!(self.expr(1));
                try!(self.expect(')'));
                Ok(e)
            }
            NUMBER(val) => {
                try!(self.next_token());
                Ok(Box::new( ast::Num { num: val }))
            }
            SYMBOL(val) => {
                //only allow math functions for now, no variables
                try!(self.next_token());
                match try!(self.peek_token()) {
                    LPAREN => {
                        try!(self.expect('('));
                        let e = try!(self.expr(1));
                        try!(self.expect(')'));
                        Ok(self.function(val,e))
                    }
                    SYMBOL(name) => {
                        match &val[..] {
                            "let" => {
                                try!(self.next_token());
                                try!(self.expect('='));
                                let expr = try!(self.expr(1));
                                Ok(Box::new( ast::Assignment { name: name, value: expr}))
                            }
                            _ => {
                                Err("Error: two consecutive symbols".to_string())
                            }
                        }
                   }
                   _ => {
                       Ok(Box::new( ast::Var { name: val }))
                   }
                }
            }
            a => {
                Err(format!("unrecognized atom: {:?}", a))
            }
        }
    }

    pub fn op (&self, op: token::Token, lhs: Box<ast::Node>, rhs: Box<ast::Node>)
            -> Box<ast::Node> {
        match op {
            ADD => {
                Box::new( ast::Add {
                    left: lhs,
                    right: rhs
                })
            }
            SUB => {
                Box::new( ast::Sub {
                    left: lhs,
                    right: rhs
                })
            }
            MUL => {
                Box::new( ast::Mul {
                    left: lhs,
                    right: rhs
                })
            }
            DIV => {
                Box::new( ast::Div {
                    left: lhs,
                    right: rhs
                })
            }
            CARET => {
                Box::new( ast::Pow {
                    base: lhs,
                    exponent: rhs
                })
            }
            o => {
                panic!("unrecognized op: {:?}", o);
            }
        }
    }

    pub fn function<'a>(&'a self, op: String, arg: Box<ast::Node>) -> Box<ast::Node> {
        match &op[..] {
            "sin" | "sine" => {
                Box::new( ast::Sin {
                    arg: arg
                })
            }
            "sqrt" | "SQRT" => {
                Box::new( ast::Sqrt {
                    arg: arg
                })
            }
            "cos" | "cosine" => {
                Box::new( ast::Cos {
                    arg: arg
                })
            }
            "print" => {
                Box::new( ast::Print {
                    arg: arg
                })
            }
            _ => {
                panic!("unrecognized function!");
            }
        }
    }
}

impl Parser {
    pub fn expect(&mut self, tok: char) -> Result<(), String> {
        try!(self.next_token());
        if self.current.to_char() != tok {
            return Err(format!("expected {:?} but found {}", tok, self.current.to_char()));
        }
        Ok(())
    }
    pub fn peek_token(&mut self) -> Result<token::Token, String> {
        if self.peeked.is_none() {
            self.peeked = Some(try!(self.lexer.next_token()));
        }
        Ok(self.peeked.clone().unwrap())
    }
    pub fn next_token(&mut self) -> Result<(), String> {
        match self.peeked {
            Some(ref mut pk) => {
                self.current = pk.clone();
            }
            None => {
                self.current = try!(self.lexer.next_token());
            }
        }
        self.peeked = None;
        Ok(())
    }
}
