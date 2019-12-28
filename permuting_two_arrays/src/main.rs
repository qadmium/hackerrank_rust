use std::io;
use std::error;
use std::str::FromStr;

fn read_line() -> Result<String, io::Error> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn read_array<T>() -> Result<Vec<T>, Box<dyn error::Error>>
where
    T: FromStr,
    T::Err: error::Error + 'static,
{
    let result: Vec<T> = read_line()?
        .trim()
        .split(' ')
        .map(&str::parse)
        .collect::<Result<Vec<T>, _>>()?;

    Ok(result)
}

fn solution(mut a: Vec<usize>, mut b: Vec<usize>, k: usize) -> bool {
    a.sort();
    b.sort_by(|left, right| left.cmp(right).reverse());

    a.iter().zip(b.iter()).map(|item| item.0 + item.1).all(|item| item >= k)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let q: usize = read_line()?.trim().parse()?;

    for _ in 0..q {
        let nk = read_array()?;
        let k = nk[1];

        let a = read_array()?;
        let b = read_array()?;

        let result = solution(a, b, k);

        println!("{}", if result {"YES"} else {"NO"});
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_0() {
        let a = vec![2, 1, 3];
        let b = vec![7, 8, 9];
        assert!(solution(a, b, 10));
    }

    #[test]
    fn test_input_1() {
        let a = vec![1, 2, 2, 1];
        let b = vec![3, 3, 3, 4];
        assert!(!solution(a, b, 5));
    }
}