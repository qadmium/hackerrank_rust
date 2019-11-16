use std::error;
use std::io;

fn read_line() -> io::Result<String> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn is_valid(s: &str) -> bool {
    let mut counter = vec![0; (b'a'..=b'z').count()];
    for ch in s.as_bytes() {
        let idx = (*ch) - b'a';
        counter[idx as usize] += 1;
    }

    let mut max_value_and_count = None;

    for &item in &counter {
        if item > 0 {
            match max_value_and_count {
                Some((value, count)) if item == value => {
                    max_value_and_count = Some((value, count + 1));
                },
                Some((value, _)) if item > value => {
                    max_value_and_count = Some((item, 1));
                },
                None => {
                    max_value_and_count = Some((item, 1));
                },
                _ => {},
            }
        }
    }

    let mut min_value_and_count = None;

    for &item in &counter {
        if item > 0 {
            match min_value_and_count {
                Some((value, count)) if item == value => {
                    min_value_and_count = Some((value, count + 1));
                },
                Some((value, _)) if item < value => {
                    min_value_and_count = Some((item, 1));
                },
                None => {
                    min_value_and_count = Some((item, 1));
                },
                _ => {},
            }
        }
    }

    let (min_value, min_count) = min_value_and_count.unwrap();
    let (max_value, max_count) = max_value_and_count.unwrap();
    let total_items = counter.iter().filter(|&&item| item > 0).count();

    if min_value == max_value {
        return true;
    }

    if min_value == 1 && min_count == 1 && total_items - max_count == 1 {
        return true;
    }

    if max_value - min_value > 1 {
        return false;
    }

    return max_count == 1;
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_line()?;
    let result = is_valid(&input);
    if result {
        println!("YES");
    } else {
        println!("NO");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid("abc"), true);
        assert_eq!(is_valid("abccdd"), false);
        assert_eq!(is_valid("abcc"), true);
        assert_eq!(is_valid("abccc"), false);
        assert_eq!(is_valid("aabbcd"), false);
        assert_eq!(is_valid("aabbc"), true);
    }

    #[test]
    fn test_is_valid_case_6() {
        assert_eq!(is_valid("xxxaabbccrry"), false);
    }
}