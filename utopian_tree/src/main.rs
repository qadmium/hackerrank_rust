use text_io::*;

fn solution(n: usize) -> usize {
    let mut result = 1;
    for i in 1..=n {
        if i % 2 == 0 {
            result += 1;
        } else {
            result *= 2;
        }
    }

    result
}

fn main() {
    for _ in 0..read!() {
        let result = solution(read!());
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        assert_eq!(solution(1), 2);
        assert_eq!(solution(4), 7);
    }
}