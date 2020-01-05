use std::error;
use std::io;
use std::i32::MAX;

fn read_line() -> Result<String, io::Error> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn read_vector() -> Result<Vec<i32>, Box<dyn error::Error>> {
    let input = read_line()?;
    let result = input.trim().split(' ').map(&str::parse).collect::<Result<Vec<i32>, _>>()?;
    Ok(result)
}

fn abs_sub(x: i32, y: i32) -> i32 {
    if x < y {
        y - x
    } else {
        x - y
    }
}

fn solution(vec: Vec<i32>) -> i32 {
    let mut min_diff = MAX;

    for i in 0..vec.len() - 1 {
        for j in i+1..vec.len() {
            let diff = abs_sub(vec[i], vec[j]);
            if diff < min_diff {
                min_diff = diff;
            }
        }
    }

    min_diff
}

fn main() -> Result<(), Box<dyn error::Error>> {
    read_line()?;
    let v = read_vector()?;
    println!("{}", solution(v));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_0() {
        assert_eq!(solution(vec![3, -7, 0]), 3);
    }

    #[test]
    fn test_sample_input_1() {
        assert_eq!(solution(vec![-59, -36, -13, 1, -53, -92, -2, -96, -54, 75]), 1);
    }

    #[test]
    fn test_sample_input_2() {
        assert_eq!(solution(vec![1, -3, 71, 68, 17]), 3);
    }
}