use std::cmp::max;
use std::error;
use std::io;

fn read_line() -> Result<String, io::Error> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn read_vec() -> Result<Vec<u8>, Box<dyn error::Error>> {
    let result = read_line()?
        .trim()
        .split_whitespace()
        .map(&str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(result)
}

fn solution(mut v: Vec<u8>) -> Option<usize> {
    v.sort();

    let folded: Vec<_> = v.into_iter()
        .fold(Vec::new(), move |mut acc, x| {
            match acc.last_mut() {
                Some((num, count)) if *num == x => {
                    *count += 1;
                },
                _ => acc.push((x, 1usize)),
            };
            acc
        });

    if folded.len() == 1 {
        return Some(folded[0].1)
    }

    folded.windows(2)
        .filter_map(|slice| match slice {
            [first, second] => Some((first, second)),
            _ => None,
        })
        .map(|((first_num, first_counter), (second_num, second_counter))| {
            if second_num - first_num == 1 {
                first_counter + second_counter
            } else {
                *max(first_counter, second_counter)
            }
        })
        .max()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    read_line()?;
    let input = read_vec()?;
    let result = solution(input).expect("More than 1 different items expected");
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = vec![6, 6, 5];
        assert_eq!(solution(input), Some(3));
    }
    
    #[test]
    fn test_2() {
        let input = vec![6, 6, 5, 2, 2, 2];
        assert_eq!(solution(input), Some(3));
    }
    
    #[test]
    fn test_3() {
        let input = vec![6, 6, 2];
        assert_eq!(solution(input), Some(2));
    }

    #[test]
    fn test_sample_input_0() {
        let input = vec![4, 6, 5, 3, 3, 1];
        assert_eq!(solution(input), Some(3));
    }

    #[test]
    fn test_sample_input_1() {
        let input = vec![1, 2, 2, 3, 1, 2];
        assert_eq!(solution(input), Some(5));
    }

    #[test]
    fn test_case_6() {
        let input = vec![66, 66, 66];
        assert_eq!(solution(input), Some(3));
    }
}