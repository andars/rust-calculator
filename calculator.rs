use std::io;

pub fn main() {
    loop {
        io::print(">> ");
        match io::stdin().read_line() {
            Ok(line) => {
                io::print("=> ");
                io::print(line);
            }
            Err(_) => {
                io::print("\n");
                break;
            }
        }
    }
}
