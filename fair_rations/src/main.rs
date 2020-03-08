use num::{Integer};
use text_io::*;

fn solution(mut v: Vec<usize>) -> Option<usize> {
    let mut result = 0;

    loop {
        let idx = v.iter().position(|item| item.is_odd());
        match idx {
            Some(idx) if idx == v.len() - 1 => return None,
            Some(idx) => {
                v[idx] += 1;
                v[idx + 1] += 1;
                result += 2;
            },
            None => return Some(result)
        }
    }
}

fn main() {
    let input = (0..read!()).map(|_| read!()).collect();
    let result = solution(input);
    match result {
        Some(result) => println!("{}", result),
        None => println!("NO"),
    }
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_sample_input_0() {
        assert_eq!(solution(vec![2, 3, 4, 5, 6]), Some(4));
    }
    
    #[test]
    fn test_sample_input_1() {
        assert_eq!(solution(vec![1, 2]), None);
    }
    
    #[test]
    fn test_example() {
        assert_eq!(solution(vec![4, 5, 7, 8]), Some(2));
    }
}