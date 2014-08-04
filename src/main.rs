use std::io;

mod ast {
    pub trait Node {
        fn eval(&self) -> f64;
    }

    pub struct Num {
        pub num: f64
    }

    impl Node for Num {
        fn eval(&self) -> f64 {
            self.num
        }
    }

    pub struct Add {
        pub left: Box<Node>,
        pub right: Box<Node>,
    }

    impl Node for Add {
        fn eval(&self) -> f64 {
            self.left.eval() + self.right.eval()
        }
    }

    pub struct Sub {
        pub left: Box<Node>,
        pub right: Box<Node>,
    }

    impl Node for Sub {
        fn eval(&self) -> f64 {
            self.left.eval() - self.right.eval()
        }
    }

    pub struct Mul {
        pub left: Box<Node>,
        pub right: Box<Node>,
    }

    impl Node for Mul {
        fn eval(&self) -> f64 {
            self.left.eval() * self.right.eval()
        }
    }

    pub struct Div {
        pub left: Box<Node>,
        pub right: Box<Node>,
    }

    impl Node for Div {
        fn eval(&self) -> f64 {
            self.left.eval() / self.right.eval()
        }
    }
}

fn parse(input: &str) -> Box<ast::Node> {
    // just return a static ast for now
    box ast::Add {
        left: box ast::Num { num: 1.0 } as Box<ast::Node>,
        right: box ast::Num { num: 2.0 } as Box<ast::Node>,
    } as Box<ast::Node>
}

fn evaluate(input: &str) -> String {
    parse(input).eval().to_string()
}

pub fn main() {
    loop {
        io::print(">> ");
        match io::stdin().read_line() {
            Ok(line) => {
                io::print("=> ");
                io::println(evaluate(line.as_slice().trim_right()).as_slice());
            }
            Err(_) => {
                io::print("\n");
                break;
            }
        }
    }
}
