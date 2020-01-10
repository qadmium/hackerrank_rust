use std::cmp::Ordering;
use std::error;
use std::io;

fn read_line() -> Result<String, io::Error> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn read_pair() -> Result<(usize, usize), Box<dyn error::Error>> {
    let input = read_line()?;
    let parsed_input = input.trim().split_whitespace().map(&str::parse).collect::<Result<Vec<usize>, _>>()?;
    match parsed_input.as_slice() {
        [first, second] => Ok((*first, *second)),
        _ => panic!("unexpected input"),
    }
}

fn solution(mut contests: Vec<(usize, usize)>, k: usize) -> i32 {
    contests.sort_by(|(left_importance, left_luck), (right_importance, right_luck)| {
        match left_importance.cmp(right_importance) {
            Ordering::Equal => left_luck.cmp(right_luck).reverse(),
            r => r,
        }
    });
    
    let mut losed_important_contests = 0;
    let mut result = 0;

    for contest in contests {
        match contest {
            (0, luck) => {
                result += luck as i32;
            },
            (1, luck) => {
                if losed_important_contests < k {
                    losed_important_contests += 1;
                    result += luck as i32;
                } else {
                    result -= luck as i32;
                }
            },
            _ => panic!("not expected input"),
        }
    }

    result
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let (n, k) = read_pair()?;

    let mut contests = Vec::with_capacity(n);
    for _ in 0..n {
        let (luck, importance) = read_pair()?;
        contests.push((importance, luck));
    }
    
    let result = solution(contests, k);
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_0() {
        let input = vec![(1, 5), (1, 2), (1, 1), (1, 8), (0, 10), (0, 5)];
        assert_eq!(solution(input, 3), 29);
    }
}