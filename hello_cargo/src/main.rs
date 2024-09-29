use std::io;

fn main() {
    println!("Hello, rust!");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("ERR");
    println!("input: {}", input);
}
