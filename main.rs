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
        pub left: ~Node,
        pub right: ~Node,
    }

    impl Node for Add {
        fn eval(&self) -> f64 {
            self.left.eval() + self.right.eval()
        }
    }

    pub struct Sub {
        pub left: ~Node,
        pub right: ~Node,
    }

    impl Node for Sub {
        fn eval(&self) -> f64 {
            self.left.eval() - self.right.eval()
        }
    }

    pub struct Mul {
        pub left: ~Node,
        pub right: ~Node,
    }

    impl Node for Mul {
        fn eval(&self) -> f64 {
            self.left.eval() * self.right.eval()
        }
    }

    pub struct Div {
        pub left: ~Node,
        pub right: ~Node,
    }

    impl Node for Div {
        fn eval(&self) -> f64 {
            self.left.eval() / self.right.eval()
        }
    }
}

fn parse(input: &str) -> ~ast::Node {
    // just return a static ast for now
    ~ast::Add {
        left: ~ast::Num { num: 1.0 } as ~ast::Node,
        right: ~ast::Num { num: 2.0 } as ~ast::Node,
    } as ~ast::Node
}

fn evaluate(input: &str) -> ~str {
    parse(input).eval().to_str()
}

pub fn main() {
    loop {
        io::print(">> ");
        match io::stdin().read_line() {
            Ok(line) => {
                io::print("=> ");
                io::println(evaluate(line.trim_right()));
            }
            Err(_) => {
                io::print("\n");
                break;
            }
        }
    }
}
