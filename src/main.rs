#![feature(box_syntax)]
#![feature(old_io)]
#![feature(collections)]

use std::old_io;
use std::collections::HashMap;
pub mod parser;

fn evaluate(input: &str) -> f64 {
    use std::f64;
    let mut p = parser::Parser::new(input);
    let mut env = HashMap::new();
    env.insert("wow", 35.0f64);
    env.insert("pi", f64::consts::PI);
    p.parse().eval(&mut env)
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
