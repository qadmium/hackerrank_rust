use std::collections::HashMap;
use std::io;

fn main() {
    let mut number = String::new();
    let mut input = String::new();

    io::stdin().read_line(&mut number).expect("cant read");
    io::stdin().read_line(&mut input).expect("cant read");

    let split = input.split(" ");
    
    let mut pairs = HashMap::new();
    
    for i in split {
        let value: i32 = i.trim().parse().expect("expected integer");
        *pairs.entry(value).or_insert(0) += 1;
    }
    
    let result: i32 = pairs.values().map(|socks_number| socks_number / 2).sum();
    
    println!("{}", result);
}