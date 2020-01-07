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

fn solution(bill: Vec<usize>, k: usize, b: isize) -> isize {
    let cost_to_exclude = bill[k];
    let total: usize = bill.into_iter().sum();
    let bill_half_without_meal = (total - cost_to_exclude) / 2;
    b - bill_half_without_meal as isize
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let (_, k) = read_pair()?;
    let bill = read_vec()?;
    let b: isize = read_line()?.trim().parse()?;
    let result = solution(bill, k, b);
    if result == 0 {
        println!("Bon Appetit");
    } else {
        println!("{}", result);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_0() {
        assert_eq!(solution(vec![3, 10, 2, 9], 1, 12), 5);
    }

}