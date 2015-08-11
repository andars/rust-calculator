use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
pub mod parser;

fn evaluate(input: &str, env: &mut HashMap<String, f64>) -> f64 {
    let mut p = parser::Parser::new(input);
    p.parse().eval(env)
}

pub fn main() {
    use std::f64;
    let mut env = HashMap::new();
    env.insert("wow".to_string(), 35.0f64);
    env.insert("pi".to_string(), f64::consts::PI);

    let mut stdin = io::stdin();


    loop {
        print!(">> ");
        io::stdout().flush().ok();

        let mut input = String::new();
        
        match stdin.read_line(&mut input) {
            Ok(_) => {
                println!("=> {}", evaluate(&input.trim_right(), &mut env));
                io::stdout().flush().ok();
            }
            Err(_) => {
                println!("");
                break;
            }
        }
    }
}
