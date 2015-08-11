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

    pub fn parse(&mut self) -> Box<ast::Node> {
        /*
        self.next_token();
        while self.current != EOF { 
            println!("{:?}", self.current);
            self.next_token();
        }
        */
        self.expr(1)
    }

    pub fn expr(&mut self, prec: usize) -> Box<ast::Node> {
        let mut lhs = self.atom();
        //println!("expr {:?}", self.current);        
        let mut rhs;
        loop {
            let curr = self.peek_token();
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
        return lhs;
    }

    pub fn atom(&mut self) -> Box<ast::Node> {
        
        match self.peek_token() {
            EOF => {return Box::new( ast::Num {num: 0f64});}
            LPAREN => {
                self.expect('(');
                let e = self.expr(1);
                self.expect(')');
                e
            }
            NUMBER(val) => {
                self.next_token();
                Box::new( ast::Num { num: val }) 
            }
            SYMBOL(val) => {
                //only allow math functions for now, no variables
                self.next_token();
                match self.peek_token() {
                    LPAREN => {
                        self.expect('(');
                        let e = self.expr(1);
                        self.expect(')');
                        self.function(val,e)
                    }
                    SYMBOL(name) => {
                        match &val[..] {
                            "let" => {
                                self.next_token();
                                self.expect('=');
                                let expr = self.expr(1);
                                Box::new( ast::Assignment { name: name, value: expr}) 
                            }
                            _ => {
                                panic!("Error: two consecutive symbols")
                            }
                        }
                   }
                   _ => {
                       Box::new( ast::Var { name: val }) 
                   }
                }
            }
            a => {
                panic!("unrecognized atom: {:?}", a);
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
    pub fn expect(&mut self, tok: char) {
        self.next_token();
        if self.current.to_char() != tok {
            panic!("expected {:?} but found {}", tok, self.current.to_char());
        }
    }
    pub fn peek_token(&mut self) -> token::Token{
        if self.peeked.is_none() {
            self.peeked = Some(self.lexer.next_token());
        }
        self.peeked.clone().unwrap()
    }
    pub fn next_token(&mut self) {
        match self.peeked {
            Some(ref mut pk) => {
                self.current = pk.clone();
            }
            None => {
                self.current = self.lexer.next_token();
            }
        }
        self.peeked = None;
    }
}
