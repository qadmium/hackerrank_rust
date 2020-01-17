use std::error;
use std::io;

fn read_line() -> Result<String, io::Error> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn read_vec() -> Result<Vec<usize>, Box<dyn error::Error>> {
    let result = read_line()?.trim().split_whitespace().map(&str::parse).collect::<Result<Vec<usize>, _>>()?;
    Ok(result)
}

fn read_pair() -> Result<(usize, usize), Box<dyn error::Error>> {
    match read_vec()?.as_slice() {
        [first, second] => Ok((*first, *second)),
        _ => panic!("non expected input"),
    }
}

fn solution(mut v: Vec<usize>, k: usize) -> usize {
    v.sort();
    v.into_iter().rev().enumerate().map(|(idx, value)| (idx / k + 1) * value).sum()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let (_, k) = read_pair()?;
    let v = read_vec()?;
    let result = solution(v, k);
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_0() {
        let input = vec![2, 5, 6];
        assert_eq!(solution(input, 3), 13);
    }

    #[test]
    fn test_sample_input_1() {
        let input = vec![2, 5, 6];
        assert_eq!(solution(input, 2), 15);
    }

    #[test]
    fn test_sample_input_2() {
        let input = vec![1, 3, 5, 7, 9];
        assert_eq!(solution(input, 3), 29);
    }
}