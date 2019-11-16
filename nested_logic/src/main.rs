use std::io;
use std::error::Error;
use std::str::FromStr;

fn read_line() -> Result<String, io::Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn read_vec<T: FromStr>() -> Result<Vec<T>, Box<dyn Error>> 
    where T::Err: Error + 'static
{
    let result = read_line()?.trim().split(' ').map(&str::parse).collect::<Result<Vec<T>, _>>()?;
    Ok(result)
}

fn calc_fine(actual_date: Vec<usize>, expected_date: Vec<usize>) -> usize{
    let (actual_day, actual_month, actual_year) = (actual_date[0], actual_date[1], actual_date[2]);
    let (expected_day, expected_month, expected_year) = (expected_date[0], expected_date[1], expected_date[2]);

    if actual_year < expected_year {
        return 0;
    }

    if actual_year > expected_year {
        return 10000;
    }

    if actual_month < expected_month {
        return 0;
    }

    if actual_month > expected_month {
        return (actual_month - expected_month) * 500;
    }

    if actual_day <= expected_day {
        return 0;
    }

    (actual_day - expected_day) * 15
}

fn main() -> Result<(), Box<dyn Error>> {
    let actual_date: Vec<usize> = read_vec()?;
    let expected_date: Vec<usize> = read_vec()?;

    let result = calc_fine(actual_date, expected_date);
    println!("{}", result);
    Ok(())
}

#[cfg(test)] 
mod tests {
    use super::*;
    #[test]
    fn test_before() {
        assert_eq!(calc_fine(vec![1, 1, 2018], vec![2, 2, 2019]), 0);
        assert_eq!(calc_fine(vec![1, 1, 2019], vec![2, 2, 2019]), 0);
        assert_eq!(calc_fine(vec![1, 1, 2019], vec![2, 1, 2019]), 0);
    }

    #[test]
    fn test_year_after() {
        assert_eq!(calc_fine(vec![1, 1, 2020], vec![2, 2, 2019]), 10000);
    }

    #[test]
    fn test_month_after() {
        assert_eq!(calc_fine(vec![1, 3, 2019], vec![2, 2, 2019]), 500);
    }

    #[test]
    fn test_days_after() {
        assert_eq!(calc_fine(vec![15, 2, 2019], vec![2, 2, 2019]), 195);
    }
}