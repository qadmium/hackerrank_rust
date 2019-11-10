use std::error;
use std::io;
use std::str::FromStr;

fn read_line() -> io::Result<String> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn read_array<T>() -> Result<Vec<T>, Box<dyn error::Error>>
where
    T: FromStr,
    T::Err: error::Error + 'static,
{
    let nd: Vec<T> = read_line()?
        .trim()
        .split(' ')
        .map(&str::parse)
        .collect::<Result<Vec<T>, _>>()?;

    Ok(nd)
}

fn merge_sort<T: PartialOrd + Copy>(input: &mut [T]) -> usize{
    let mut output = input.to_vec();
    merge_int(&mut output, input)
}

fn merge_int<'a, T: PartialOrd + Copy>(from: &'a mut [T], merge_to: &'a mut [T]) -> usize {
    debug_assert!(from.len() == merge_to.len());
    let mut result = 0usize;
    let input_len = from.len();

    if input_len <= 1 {
        return result;
    }

    let mid = input_len / 2;

    result += merge_int(&mut merge_to[0..mid], &mut from[0..mid]);
    result += merge_int(&mut merge_to[mid..input_len], &mut from[mid..input_len]);
    
    let left_part = &from[0..mid];
    let right_part = &from[mid..input_len];
    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut out_idx = 0;
    while left_idx < left_part.len() || right_idx < right_part.len() {
        if left_idx >= left_part.len() {
            merge_to[out_idx] = right_part[right_idx];
            right_idx += 1;
        } else if right_idx >= right_part.len() || left_part[left_idx] <= right_part[right_idx] {
            merge_to[out_idx] = left_part[left_idx];
            left_idx += 1;
        } else {
            merge_to[out_idx] = right_part[right_idx];
            right_idx += 1;
            let left_on_left_part = left_part.len() - left_idx;
            result += left_on_left_part;
        }

        out_idx += 1;
    }
    result
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let d: usize = read_line()?.trim().parse()?;

    for _ in 0..d {
        read_line()?;
        let mut vec: Vec<usize> = read_array()?;

        println!("{}", merge_sort(&mut vec));
    }
    Ok(())
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_one_items() {
        let mut input = vec![2];
        let expected = vec![2];

        merge_sort(&mut input);
        assert_eq!(input[0], expected[0]);
    }

    #[test]
    fn test_merge_two_items() {
        let mut input = vec![2, 1];
        let expected = vec![1, 2];

        merge_sort(&mut input);
        assert_eq!(input, expected);

        let mut input = vec![1, 2];
        let expected = vec![1, 2];

        merge_sort(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_merge_three_items() {
        let mut input = vec![3, 2, 1];
        let expected =  vec![1, 2, 3];

        merge_sort(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_merge_no_sort() {
        let mut input = vec![1, 1, 1, 2, 2];
        let expected =  vec![1, 1, 1, 2, 2];

        merge_sort(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_merge_five_items() {
        let mut input = vec![2, 1, 3, 1, 2];
        let expected =  vec![1, 1, 2, 2, 3];

        assert_eq!(merge_sort(&mut input), 4);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_sample_1_2() {
        let mut input = vec![7, 5, 3, 1];
        let expected =  vec![1, 3, 5, 7];

        assert_eq!(merge_sort(&mut input), 6);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_sample_1_1() {
        let mut input = vec![1, 5, 3, 7];
        let expected =  vec![1, 3, 5, 7];

        assert_eq!(merge_sort(&mut input), 1);
        assert_eq!(input, expected);
    }
}