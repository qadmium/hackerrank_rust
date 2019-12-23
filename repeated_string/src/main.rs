use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("cant read");
    let input_string = input.trim().to_string();

    input.clear();
    io::stdin().read_line(&mut input).expect("cant read");
    let n: usize = input.trim().parse().expect("expected int");
    
    let string_length = input_string.len();
    let a_letters_in_string = input_string.chars().filter(|c| *c == 'a').count();

    let repetitions = n / string_length;
    let left_characters = n % string_length;
    let result = input_string.chars().take(left_characters).filter(|c| *c == 'a').count();
    
    println!("{}", result + repetitions * a_letters_in_string);
}