#![feature(box_syntax)]
#![feature(old_io)]
#![feature(collections)]

use std::old_io;
pub mod parser;

fn evaluate(input: &str) -> f64 {
    let mut p = parser::Parser::new(input);
    p.parse().eval()
}

pub fn main() {
    loop {
        old_io::print(">> ");
        match old_io::stdin().read_line() {
            Ok(line) => {
                println!("=> {}", evaluate(line.as_slice().trim_right().as_slice()));
            }
            Err(_) => {
                old_io::print("\n");
                break;
            }
        }
    }
}
