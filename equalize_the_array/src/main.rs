use std::collections::HashMap;
use text_io::*;

fn build_counter(v: Vec<u8>) -> HashMap<u8, usize> {
    let mut result = HashMap::new();
    for item in v {
        *result.entry(item).or_default() += 1;
    }
    result
}

fn solution(v: Vec<u8>) -> usize {
    let len = v.len();
    let counter = build_counter(v);
    let result = counter.values().max().unwrap();
    len - result
}

fn main() {
    let input = (0..read!()).map(|_| read!()).collect();
    let result = solution(input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_sample_input() {
        assert_eq!(solution(vec![3, 3, 2, 1, 3]), 2);
    }
}