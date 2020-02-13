use std::collections::{HashMap, HashSet};
use text_io::*;

fn abs_diff(left: usize, right: usize) -> usize {
    if left < right {
        right - left
    } else {
        left - right
    }
}

fn solution(v: Vec<Vec<usize>>) -> bool {
    let mut ball_types_by_len_counter = HashMap::new();
    for ball_type in 0..v.len() {
        let count: usize = v.iter().map(|container| container[ball_type]).sum();
        *ball_types_by_len_counter.entry(count).or_insert(0) += 1;
    }
    
    let mut containers_capacities = HashMap::new();

    for container in v {
        let capacity = container.iter().sum();
        *containers_capacities.entry(capacity).or_insert(0) += 1;
    }

    let ball_counts: HashSet<_> = ball_types_by_len_counter.keys().collect();
    let countainers_lens: HashSet<_> = containers_capacities.keys().collect();
    
    if ball_counts != countainers_lens {
        return false;
    }

    for (ball_count, &count) in ball_types_by_len_counter.iter() {
        match containers_capacities.get(ball_count).map(|&value| abs_diff(value, count)) {
            None => return false,
            Some(0) => {},
            _ => return false,
        }
    }

    true
}

fn main() {
    for _ in 0..read!() {
        let n = read!();
        let input = (0..n).map(|_| {
            (0..n).map(|_| read!()).collect()
        }).collect();
        let result = solution(input);

        println!("{}", if result {"Possible"} else {"Impossible"});
    }
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_sample_input_0() {
        let input = vec![
            vec![1, 1],
            vec![1, 1],
        ];
        assert_eq!(solution(input), true);

        let input = vec![
            vec![0, 2],
            vec![1, 1],
        ];
        assert_eq!(solution(input), false);
    }

    
    #[test]
    fn test_sample_input_1() {
        let input = vec![
            vec![1, 3, 1],
            vec![2, 1, 2],
            vec![3, 3, 3],
        ];

        assert_eq!(solution(input), false);

        let input = vec![
            vec![0, 2, 1],
            vec![1, 1, 1],
            vec![2, 0, 0],
        ];

        assert_eq!(solution(input), true);
    }
}