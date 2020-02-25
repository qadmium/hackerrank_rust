use std::collections::HashMap;
use text_io::*;

fn solution(a: Vec<usize>) -> Option<usize> {
    let mut values_indexes = HashMap::new();

    for (idx, item) in a.into_iter().enumerate() {
        values_indexes.entry(item).or_insert(Vec::new()).push(idx);
    }

    values_indexes
        .values()
        .map(|indexes| 
            indexes
                .windows(2)
                .map(|window| window.iter().fold(0, |acc, value| value - acc))
        )
        .flatten()
        .min()
}

fn main() {
    let input = (0..read!()).map(|_| read!()).collect();
    let result = solution(input);
    println!("{}", result.map(|v| v.to_string()).unwrap_or("-1".to_owned()));
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_sample_input() {
        assert_eq!(solution(vec![7, 1, 3, 4, 1, 7]), Some(3));
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(solution(vec![]), None);
    }

    #[test]
    fn test_empty_result() {
        assert_eq!(solution(vec![1, 2, 4]), None);
    }
}


