use text_io::*;

fn solution(s: &str, k: u8) -> String {
    assert!(s.is_ascii());

    let mut result = Vec::with_capacity(s.len());
    let alphabet_size = b'z' - b'a' + 1;

    for ch in s.bytes() {
        if b'a' <= ch && ch <= b'z' {
            let ch_offset = ch - b'a';
            let new_ch_offset = (ch_offset + k) % alphabet_size;
            result.push(new_ch_offset + b'a');
        } else if b'A' <= ch && ch <= b'Z' {
            let ch_offset = ch - b'A';
            let new_ch_offset = (ch_offset + k) % alphabet_size;
            result.push(new_ch_offset + b'A');
        } else {
            result.push(ch);
        }
    }

    String::from_utf8(result).unwrap()
}

fn main() {
    let _: u8 = read!();
    let input: String = read!();
    let result = solution(&input, read!());
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_sample_input() {
        assert_eq!(solution("middle-Outz", 2), "okffng-Qwvb".to_owned());
    }
}
