#![feature(box_syntax)]
#![feature(old_io)]
#![feature(collections)]

use std::old_io;
use std::collections::HashMap;
pub mod parser;

fn evaluate(input: &str, env: &mut HashMap<String, f64>) -> f64 {
    let mut p = parser::Parser::new(input);
    p.parse().eval(env)
}

pub fn main() {
    use std::f64;
    let mut env = HashMap::new();
    env.insert(String::from_str("wow"), 35.0f64);
    env.insert(String::from_str("pi"), f64::consts::PI);
    loop {
        old_io::print(">> ");
        match old_io::stdin().read_line() {
            Ok(line) => {
                println!("=> {}", evaluate(&line.trim_right(), &mut env));
            }
            Err(_) => {
                old_io::print("\n");
                break;
            }
        }
    }
}
