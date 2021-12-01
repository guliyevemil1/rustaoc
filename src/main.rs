mod day1;

use std::io;
use std::io::Read;

fn main() {
    let mut input : Vec<String> = vec![];
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        if buffer == "\n" {
            break;
        }
        input.push(String::from(buffer.trim()));
    }
    println!("{}", day1::run2(input))
}
