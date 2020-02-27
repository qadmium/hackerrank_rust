use std::collections::HashMap;
use text_io::*;

fn build_counter(v: Vec<usize>) -> HashMap<usize, usize> {
    let mut result = HashMap::new();

    for item in v {
        *result.entry(item).or_default() += 1;
    }

    result
}

fn solution(v: Vec<usize>, d: usize) -> usize {
    let counter = build_counter(v);
    let mut result = 0;

    for (i_item, i_count) in counter.iter() {
        let j_item = i_item + d;
        let j_count = counter.get(&j_item).unwrap_or(&0);
        let k_item = j_item + d;
        let k_count = counter.get(&k_item).unwrap_or(&0);

        result += i_count * j_count * k_count;
    }

    result
}

fn main() {
    let n = read!();
    let d = read!();
    let input = (0..n).map(|_| read!()).collect();
    let result = solution(input, d);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_sample_input() {
        assert_eq!(solution(vec![2, 2, 3, 4, 5], 1), 3);
        assert_eq!(solution(vec![1, 2, 4, 5, 7, 8, 10], 3), 3);
    }
}
