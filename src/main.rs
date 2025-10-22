use std::io::{self};

mod parse;

fn main() -> io::Result<()> {
    for line in io::stdin().lines() {
        let packet = parse::parse(line.unwrap());
        println!("{:?}", packet);
    }

    return Ok(());
}
