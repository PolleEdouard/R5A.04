use std::collections::HashSet;
use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut free_food_days = HashSet::new();

    for _ in 0..t {

            let line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            let s: usize = parts.next().unwrap().parse().unwrap();
            let t: usize = parts.next().unwrap().parse().unwrap();
            

        for i in s..=t {
            free_food_days.insert(i);
        }
    
    }
    println!("{}", free_food_days.len());
}
