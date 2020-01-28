use text_io::*;

struct DigitsIterator {
    n: u64,
}

impl Iterator for DigitsIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            return None;
        }

        let result = self.n % 10;
        self.n /= 10;
        Some(result)
    }
}

fn digits_iter(n: u64) -> DigitsIterator {
    DigitsIterator{n}
}

fn solution(n: u64) -> usize {
    digits_iter(n).filter(|&digit| digit != 0).filter(|digit| n % digit == 0).count()
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
    fn test_sample_input_0() {
        assert_eq!(solution(12), 2);
        assert_eq!(solution(1012), 3);
    }
}
