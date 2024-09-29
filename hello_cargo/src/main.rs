use std::io;

fn main() {
    let cond = 2 < 3;
    println!("umm {}", cond);

    println!("Hello, rust!");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("ERR");
    println!("input: {}", input);
}
