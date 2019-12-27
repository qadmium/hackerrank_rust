use std::borrow::ToOwned;
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use std::io;

fn read_line() -> io::Result<String> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn build_counter<T>(s: &[T]) -> HashMap<T, usize> where T: Hash + Eq + ToOwned<Owned=T> {
    let mut result: HashMap<T, usize> = HashMap::new();
    for c in s {
        result.entry(c.to_owned()).and_modify(|v| *v+=1).or_insert(1);
    }
    result
}

fn count_pairs<T>(slice: &[T]) -> usize where T: PartialEq {
    let mut result = 0;
    for (idx, current) in slice.iter().enumerate() {
        result += slice.iter().skip(idx + 1).filter(|&item| *item == *current).count();
    }
    result
}

fn check_string(val: &str) -> usize {
    let mut result = 0;
    let chars: Vec<_> = val.chars().collect();
    for len in 1..chars.len() {
        let anagrams: Vec<_> = chars.windows(len).map(build_counter).collect();
        result += count_pairs(&anagrams);        
    }
    result
}

fn main() -> Result<(), io::Error> {
    let input = read_line()?;
    let n: usize = input.trim().parse().map_err(|_| io::ErrorKind::Other)?;

    for _ in 0..n {
        let input_string = read_line()?;
        println!("{}", check_string(&input_string));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_counter() {
        let v: Vec<_> = "aabbccaab".chars().collect();
        let actual = build_counter(&v);
        let expected: HashMap<_, _> = [
            ('a', 4),
            ('b', 3),
            ('c', 2),
        ].iter().cloned().collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_count_pairs() {
        assert_eq!(count_pairs(&[2]), 0);
        assert_eq!(count_pairs(&[2, 2]), 1);
        assert_eq!(count_pairs(&[2, 2, 2]), 3);
    }

    #[test]
    fn test_check_string() {
        assert_eq!(check_string("abba"), 4);
        assert_eq!(check_string("abcd"), 0);
        assert_eq!(check_string("ifailuhkqq"), 3);
        assert_eq!(check_string("kkkk"), 10);
    }
}