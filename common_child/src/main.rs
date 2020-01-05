use std::error;
use std::io;
use std::mem;

fn read_line() -> Result<String, io::Error> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn lcs_length(x: &str, y: &str) -> usize {
    let y_chars: Vec<char> = y.chars().collect();
    let x_chars: Vec<char> = x.chars().collect();
    
    let mut current = vec![0; y_chars.len() + 1];
    let mut prev = current.to_owned();

    for i in 0..x_chars.len() {
        mem::swap(&mut prev, &mut current);
        
        for j in 0..y_chars.len() {
            current[j+1] = if y_chars[j] == x_chars[i] {
                prev[j] + 1
            } else if current[j] > prev[j+1] {
                current[j]
            } else {
                prev[j+1]
            };
        }
    }

    current[current.len() - 1]
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let first_string = read_line()?;
    let second_string = read_line()?;
    println!("{}", lcs_length(first_string.trim(), second_string.trim()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_0() {
        assert_eq!(lcs_length("HARRY", "SALLY"), 2);
    }

    #[test]
    fn test_sample_input_1() {
        assert_eq!(lcs_length("AA", "BB"), 0);
    }

    #[test]
    fn test_sample_input_2() {
        assert_eq!(lcs_length("SHINCHAN", "NOHARAAA"), 3);
    }

    #[test]
    fn test_sample_input_3() {
        assert_eq!(lcs_length("ABCDEF", "FBDAMN"), 2);
    }
}