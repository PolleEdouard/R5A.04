use std::io::BufRead;
fn main() {
    let stdin = std::io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let mut result = Vec::new();

    for (i,line) in lines.iter().enumerate(){
        if line.contains("FBI"){
            result.push(i+1);
        }
    }
    if result.is_empty(){
        println!("HE GOT AWAY!");
    }else{
        for i in result{
            print!("{} ",i);
        }
    }
    println!();
}
