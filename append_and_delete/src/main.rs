use text_io::*;

fn solution(s: &[u8], t: &[u8], k: usize) -> bool {
    let mut i = 0;
    while i < t.len() && i < s.len() && t[i] == s[i] {
        i += 1;
    }

    let min_steps = t.len() - i + s.len() - i;

    if k < min_steps {
        return false;
    }
    
    if k == min_steps {
        return true;
    }

    if k >= t.len() + s.len() {
        return true;
    }

    (k - min_steps) % 2 == 0
}

fn main() {
    let s: String = read!();
    let t: String = read!();
    let result = solution(s.as_bytes(), t.as_bytes(), read!());
    println!("{}", if result {"Yes"} else {"No"});
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_sample_input_0() {
        assert_eq!(solution("hackerhappy".as_bytes(), "hackerrank".as_bytes(), 9), true);
    }

    #[test]
    fn test_sample_input_1() {
        assert_eq!(solution("aba".as_bytes(), "aba".as_bytes(), 7), true);
    }

    #[test]
    fn test_sample_input_2() {
        assert_eq!(solution("ashley".as_bytes(), "ash".as_bytes(), 2), false);
    }

    #[test]
    fn test_case_5() {
        assert_eq!(solution("y".as_bytes(), "yu".as_bytes(), 1), true);
        assert_eq!(solution("y".as_bytes(), "yu".as_bytes(), 2), false);
        assert_eq!(solution("y".as_bytes(), "yu".as_bytes(), 3), true);
    }
}
