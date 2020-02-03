use std::cmp::Ordering;
use text_io::*;

fn solution(v: Vec<(usize, usize, usize)>) -> Vec<usize> {
    let mut finish_times: Vec<_> = v.into_iter().map(|(i, di, ti)| (i, di + ti)).collect();
    finish_times.sort_by(|(left_i, left_fti), (right_i, right_fti)| {
        let result = left_fti.cmp(right_fti);
        if result == Ordering::Equal {
            return left_i.cmp(right_i)
        }

        result
    });

    finish_times.into_iter().map(|(i, _)| i).collect()
}

fn main() {
    let input: Vec<_> = (0..read!()).map(|i| (i + 1, read!(), read!())).collect();
    let result = solution(input);
    println!("{}", result.into_iter().fold(String::new(), |acc, value| acc + &value.to_string() + " "));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sample_input_0() {
        assert_eq!(solution(vec![(1, 1, 3), (2, 2, 3), (3, 3, 3)]), vec![1, 2, 3]);
    }

    #[test]
    fn test_sample_input_1() {
        assert_eq!(solution(vec![(1, 8, 1), (2, 4, 2), (3, 5, 6), (4, 3, 1), (5, 4, 3)]), vec![4, 2, 5, 1, 3]);
    }
}