use std::collections::HashSet;
use text_io::*;

fn solution(v: Vec<i64>, k: i64) -> usize {
    let mut watched_items = HashSet::new();
    let mut result = 0;
    for item in v {
        let target = item - k;
        if watched_items.contains(&target) {
            result += 1;
        }
        let target2 = item + k;
        if watched_items.contains(&target2) {
            result += 1;
        }

        watched_items.insert(item);
    }

    result
}

fn main() {
    let n: usize = read!();
    let k: i64 = read!();
    let input = (0..n).map(|_| read!()).collect();
    let result = solution(input, k);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        assert_eq!(solution(vec![1, 5, 3, 4, 2], 2), 3);
    }
}
