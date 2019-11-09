use std::borrow::ToOwned;
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use std::io;

use num::Integer;

fn read_line() -> io::Result<String> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn build_counter<T, U>(seq: U) -> HashMap<T, usize> where
    U: Iterator<Item=T>,
    T: Hash + Eq + ToOwned<Owned=T> {
    let mut result: HashMap<T, usize> = HashMap::new();
    for c in seq {
        result.entry(c.to_owned()).and_modify(|v| *v+=1).or_insert(1);
    }
    result
}

fn check_string(val: &str) -> bool {
    build_counter(val.chars()).values().filter(|v| v.is_odd()).count() <= 1
}

fn main() -> Result<(), io::Error> {
    let input = read_line()?;
    let result = if check_string(&input) {
        "YES"
    } else {
        "NO"
    };

    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_counter() {
        let actual = build_counter("aabbccaab".chars());
        let expected: HashMap<_, _> = [
            ('a', 4),
            ('b', 3),
            ('c', 2),
        ].iter().cloned().collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_check_string() {
        assert_eq!(check_string("abba"), true);
        assert_eq!(check_string("abcd"), false);
        assert_eq!(check_string("ifailuhkqq"), false);
        assert_eq!(check_string("aaabbbb"), true);
    }
}