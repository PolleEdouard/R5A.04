use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("zougougoula");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    let right_number: i32 = parts[1].parse().expect( "Not a number");
    let left_number: i32 = parts[0].parse().expect( "Not a number");
        if (right_number == 0) && (left_number == 0) {
            println!("Not a moose");
        } else if right_number == left_number {
            println!("Even {}", right_number * 2);
        } else {
            let max = std::cmp::max(right_number, left_number);
            println!("Odd {}", max * 2);
        }
}
