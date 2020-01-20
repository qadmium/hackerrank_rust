use std::error;
use text_io::{read, try_read, try_scan}; // for compatibility with hackerrank's rust

fn read_heights() -> Vec<u8> {
    let len = b'z' - b'a' + 1;
    (0..len).map(|_| read!()).collect()
}

fn solution(s: &str, v: Vec<u8>) -> Option<usize> {
    assert!(s.is_ascii());
    let max_height = s.bytes().map(|ch| v[(ch - b'a') as usize]).max();
    max_height.map(|v| v as usize * s.len())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let heights = read_heights();
    let s: String = read!();
    let result = solution(&s, heights).ok_or("bad input")?;
    println!("{}", result);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_input_0() {
        let input = vec![1, 3, 1, 3, 1, 4, 1, 3, 2, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5];
        assert_eq!(solution("abc", input), Some(9));
    }

}