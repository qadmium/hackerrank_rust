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

fn solution(k: usize, ar: Vec<usize>) -> usize {
    let mut result = 0;
    for i in 0..ar.len() - 1 {
        for j in i+1..ar.len() {
            if (ar[j] + ar[i]) % k == 0 {
                result += 1;
            }
        }
    }
    result
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let nk: Vec<usize> = read_array()?;
    let ar: Vec<usize> = read_array()?;
    let k = nk[1];
    let result = solution(k, ar);
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_case() {
        let ar = vec![1, 3, 2, 6, 1, 2];
        let k = 3;
        assert_eq!(solution(k, ar), 5);
    }
}