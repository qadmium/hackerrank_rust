use std::io;
use std::error;

fn read_line() -> Result<String, io::Error> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn solution(s: &str) -> usize {
    const PAT: &str = "010";
    const PAT_LEN: usize = PAT.len();
    let mut s = s.to_string();
    let mut result = 0;

    while let Some(idx) = s.find(PAT) {
        let next_ch = idx + PAT_LEN;
        let replacement = if next_ch >= s.len() || s[idx..=next_ch].ends_with('0') {
            "000"
        } else {
            "011"
        };
        s.replace_range(idx..idx+PAT_LEN, replacement);
        result += 1;
    }

    result
}

fn main() -> Result<(), Box<dyn error::Error>> {
    read_line()?;
    let input = read_line()?;
    let result = solution(input.trim());
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_0() {
        assert_eq!(solution("0101010"), 2);
    }

    #[test]
    fn test_input_1() {
        assert_eq!(solution("01100"), 0);
    }

    #[test]
    fn test_input_3() {
        assert_eq!(solution("0100101010"), 3);
    }
}
