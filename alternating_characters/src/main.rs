use std::error;
use std::io;

fn read_line() -> io::Result<String> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let num_of_lines: usize = read_line()?.trim().parse()?;

    for _ in 0..num_of_lines {
        let input = read_line()?;
        let mut result = 0;
        let mut prev_character = None;
        for character in input.chars() {
            if prev_character == Some(character) {
                result += 1;
            }
            prev_character = Some(character);
        }
        println!("{}", result);
    }
    
    Ok(())
}