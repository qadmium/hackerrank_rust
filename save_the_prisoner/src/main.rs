use text_io::*;

fn solution(n: usize, m: usize, s: usize) -> usize {
    let result = (m + s - 1) % n;
    if result == 0 {
        n
    } else {
        result
    }
}

fn main() {
    for _ in 0..read!() {
        let result = solution(read!(), read!(), read!());
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solution(3, 3, 1), 3);
    }

    #[test]
    fn test_sample_input_0() {
        assert_eq!(solution(5, 2, 1), 2);
        assert_eq!(solution(5, 2, 2), 3);
    }

    #[test]
    fn test_sample_test_case_1() {
        assert_eq!(solution(7, 19, 2), 6);
        assert_eq!(solution(3, 7, 3), 3);
    }
}
