use std::borrow::ToOwned;
use std::cmp::{Eq, max, min};
use std::collections::{HashMap, HashSet};
use std::error;
use std::hash::Hash;
use std::io;

fn read_line() -> io::Result<String> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn build_counter<T>(s: &[T]) -> HashMap<T, usize> where T: Hash + Eq + ToOwned<Owned=T> {
    let mut result = HashMap::new();
    for item in s {
        result.entry(item.to_owned()).and_modify(|v| *v += 1).or_insert(1);
    }

    result
}

fn hash_map_symmetric_diff<T: Hash + Eq + ToOwned<Owned=T>>(left: &HashMap<T, usize>, right: &HashMap<T, usize>) -> HashMap<T, usize> {
    let mut result: HashMap<T, _> = HashMap::new();

    let left_keys: HashSet<_> = left.keys().collect();
    let right_keys: HashSet<_> = right.keys().collect();

    for &item in left_keys.symmetric_difference(&right_keys) {
        let value = left.get(item).or_else(|| right.get(item)).expect("key should be either in left or right map");
        result.insert(item.to_owned(), *value);
    }

    result
}

fn hash_map_intersection<T: Hash + Eq + ToOwned<Owned=T>>(left: &HashMap<T, usize>, right: &HashMap<T, usize>) -> HashMap<T, usize> {
    let mut result: HashMap<T, _> = HashMap::new();

    let left_keys: HashSet<_> = left.keys().collect();
    let right_keys: HashSet<_> = right.keys().collect();

    for &item in left_keys.intersection(&right_keys) {
        let left_value = left.get(item).expect("key should be in both maps");
        let right_value = right.get(item).expect("key should be in both maps");

        result.insert(item.to_owned(), max(left_value, right_value) - min(left_value, right_value));
    }

    result
}

fn solution(s1: &str, s2: &str) -> usize {
    assert!(s1.is_ascii());
    let s1_counter = build_counter(s1.as_bytes());

    assert!(s2.is_ascii());
    let s2_counter = build_counter(s2.as_bytes());

    let diff: usize = hash_map_symmetric_diff(&s1_counter, &s2_counter).values().sum();
    let intersection_diff: usize = hash_map_intersection(&s1_counter, &s2_counter).values().sum();

    diff + intersection_diff
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let s1 = read_line()?;
    let s2 = read_line()?;
    
    let result = solution(s1.trim(), s2.trim());
    println!("{}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_0() {
        assert_eq!(solution("abc", "cde"), 4)
    }
}
