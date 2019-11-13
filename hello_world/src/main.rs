fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    println!("Hello, World.");
    println!("{}", input);
}
