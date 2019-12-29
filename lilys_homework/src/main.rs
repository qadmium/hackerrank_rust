use std::cmp::{min, Ordering};
use std::error;
use std::io;
use std::str::FromStr;

fn read_line() -> Result<String, io::Error> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn read_array<T: FromStr>() -> Result<Vec<T>, Box<dyn error::Error>> 
where T::Err: error::Error + 'static
{
    let result = read_line()?.trim().split(' ').map(&str::parse).collect::<Result<Vec<T>, _>>()?;
    Ok(result)
}

fn swaps_with_selection_sort<F: Fn(usize, &usize) -> Ordering>(mut arr: Vec<usize>, by: F) -> usize {
    let mut result = 0;
    for i in 0..arr.len() - 1 {
        let idx_to_swap = &arr[i..arr.len()]
            .iter()
            .enumerate()
            .min_by(|(_, &a), (_, b)| by(a, b))
            .map(|(idx, _)| i + idx)
            .expect("min or max always exists");

        if *idx_to_swap != i {
            arr.swap(i, *idx_to_swap);
            result += 1;
        }
    }
    result
}

fn solution(arr: Vec<usize>) -> usize {
    let asc_sort_result = swaps_with_selection_sort(arr.to_owned(), |a, b| a.cmp(b));
    let desc_sort_result = swaps_with_selection_sort(arr, |a, b| a.cmp(b).reverse());
    min(asc_sort_result, desc_sort_result)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    read_line()?;
    let arr: Vec<usize> = read_array()?;
    let result = solution(arr);
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_0() {
        let input = vec![7, 15, 12, 3];
        assert_eq!(solution(input), 2);
    }

    #[test]
    fn test_input_1() {
        let input = vec![2, 5, 3, 1];
        assert_eq!(solution(input), 2);
    }

    #[test]
    fn test_case_1() {
        let input = vec![3, 4, 2, 5, 1];
        assert_eq!(solution(input), 2);
    }
}
