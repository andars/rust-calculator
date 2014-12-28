
use std::io;
pub mod parser;

fn evaluate(input: &str) -> f64 {
    let mut p = parser::Parser::new(input);
    p.parse().eval()
}

pub fn main() {
    loop {
        io::print(">> ");
        match io::stdin().read_line() {
            Ok(line) => {
                println!("=> {:f}", evaluate(line.as_slice().trim_right().as_slice()));
            }
            Err(_) => {
                io::print("\n");
                break;
            }
        }
    }
}
