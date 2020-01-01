use std::io;
use std::error;

fn read_line() -> Result<String, io::Error> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn abs_sub(a: u8, b: u8) -> u8 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn solution(s: &str) -> usize {
    assert!(s.is_ascii());
    let ascii_string = s.as_bytes();
    let mut result: usize = 0;

    for i in 0..(ascii_string.len() / 2) {
        let left_ch = ascii_string[i];
        let right_ch = ascii_string[ascii_string.len() - 1 - i];
        result += abs_sub(left_ch, right_ch) as usize;
    }

    result
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let n: usize = read_line()?.trim().parse()?;

    for _ in 0..n {
        let input = read_line()?;
        let result = solution(input.trim());
        println!("{}", result);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_0() {
        assert_eq!(solution("abc"), 2);
    }

    #[test]
    fn test_sample_input_1() {
        assert_eq!(solution("abcba"), 0);
    }

    #[test]
    fn test_sample_input_2() {
        assert_eq!(solution("abcd"), 4);
    }
}