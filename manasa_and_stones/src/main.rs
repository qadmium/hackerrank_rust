use std::cmp::{min, max};
use std::iter::repeat;
use text_io::*;

fn solution(n: usize, a: usize, b: usize) -> Vec<usize> {
    if a == b {
        return vec![a * (n - 1)];
    }
    let min_number = min(a, b);
    let max_number = max(a, b);
    (0..n).map(|i| {
        let min_sum: usize = repeat(min_number).take(n - 1 - i).sum();
        let max_sum: usize = repeat(max_number).take(i).sum();
        min_sum + max_sum
    }).collect()
}

fn main() {
    for _ in 0..read!() {
        let result: Vec<_> = solution(read!(), read!(), read!())
            .into_iter()
            .map(|sum| sum.to_string())
            .collect();

        println!("{}", result.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_sample_input_0() {
        assert_eq!(solution(3, 1, 2), vec![2, 3, 4]);
    }

    #[test]
    fn test_sample_input_1() {
        assert_eq!(solution(4, 10, 100), vec![30, 120, 210, 300]);
    }

    #[test]
    fn test_case_5_3() {
        assert_eq!(solution(73, 25, 25), vec![1800]);
    }

    #[test]
    fn test_case_5_5() {
        assert_eq!(solution(5, 3, 23), vec![12, 32, 52, 72, 92]);
    }
}
