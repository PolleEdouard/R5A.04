use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    let carrots = parts[1].parse::<i32>()

    println!("{}",carrots);
}
