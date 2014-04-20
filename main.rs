use std::io;

fn evaluate(input: &str) -> ~str {
    // do nothing with the input for now
    input.to_owned()
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
