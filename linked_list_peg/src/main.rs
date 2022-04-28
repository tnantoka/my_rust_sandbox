mod list;
mod parser;

use std::io::stdin;

use list::List;
use parser::manager;

fn main() {
    let mut list = List::new();
    loop {
        let message = input();
        if message.is_empty() {
            break;
        }
        let result = manager::eval(&message, &mut list).unwrap();
        if !result.is_empty() {
            println!("{}", result)
        }
    }
}

fn input() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    String::from(buf.trim())
}
