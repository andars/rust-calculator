
use std::io;
pub mod parser;

fn evaluate(input: &str) -> String {
    parser::parse(input).eval().to_string()
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
