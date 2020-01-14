use std::error;
use std::io;

fn read_line() -> Result<String, io::Error> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

pub trait AdjacentPairs {
    type Iter: Iterator;

    fn adjacent_pairs(self) -> AdjacentPairsState<Self::Iter>;
}

impl<T> AdjacentPairs for T where T: Iterator + Clone {
    type Iter = T;

    fn adjacent_pairs(mut self) -> AdjacentPairsState<Self::Iter> {
        let cloned = self.clone();
        self.next();
        AdjacentPairsState{iter: self, prev_iter: cloned} 
    }
}

pub struct AdjacentPairsState<T: Iterator> {
    iter: T,
    prev_iter: T,
}

impl<T: Iterator> Iterator for AdjacentPairsState<T> {
    type Item = (T::Item, T::Item);
    fn next(&mut self) -> Option<Self::Item> {
        match (self.prev_iter.next(), self.iter.next()) {
            (_, None) => None,
            (Some(prev), Some(next)) => Some((prev, next)),
            _ => panic!("strange state, prev is empty but next has value"),
        }
    }
}

fn abs_sub(x: u8, y: u8) -> u8 {
    if x > y {
        x - y
    } else {
        y - x
    }
}

fn solution(s: &str) -> bool {
    assert!(s.is_ascii());

    let pairs = s.bytes().adjacent_pairs().map(|(left, right)| abs_sub(left, right));
    let rev_pairs = s.bytes().rev().adjacent_pairs().map(|(left, right)| abs_sub(left, right));

    pairs.eq(rev_pairs)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let n: usize = read_line()?.trim().parse()?;

    for _ in 0..n {
        let input = read_line()?;
        let result = solution(input.trim());
        let output = if result {"Funny"} else {"Not Funny"};
        println!("{}", output);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_adjacent_pairs_adaptor() {
        let v = vec![1, 2, 3];
        let mut iter = v.iter().adjacent_pairs();
        assert_eq!(iter.next(), Some((&1, &2)));
        assert_eq!(iter.next(), Some((&2, &3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_adjacent_pairs_adaptor_on_one_elem() {
        let v = vec![1];
        let mut iter = v.iter().adjacent_pairs();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_sample_input() {
        assert_eq!(solution("acxz"), true);
        assert_eq!(solution("bcxz"), false);
    }
}
