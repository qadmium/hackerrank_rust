use std::error;
use std::io;
use std::str::FromStr;
mod sliding_median;

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

fn solution(d: usize, arr: &[usize]) -> usize {
    use sliding_median::SlidingMedian;
    arr.iter()
        .sliding_median(d, 200)
        .enumerate()
        .take_while(|(idx, _)| idx + d < arr.len())
        .map(|(idx, median)| (arr[idx + d], median))
        .filter(|(day_expenditure, median)| *day_expenditure as f64 >= median * 2.0)
        .count()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let d: usize = read_array()?[1];
    let arr: Vec<usize> = read_array()?;

    println!("{}", solution(d, &arr));
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(3, &[1, 1]), 0);
        assert_eq!(solution(3, &[1, 1, 1]), 0);
        assert_eq!(solution(3, &[1, 1, 1, 0]), 0);
        assert_eq!(solution(3, &[1, 1, 1, 2]), 1);

        assert_eq!(solution(5, &[2, 3, 4, 2, 3, 6, 8, 4, 5]), 2);
        assert_eq!(solution(4, &[1, 2, 3, 4, 4]), 0);
    }
}