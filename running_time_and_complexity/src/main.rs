use std::error;
use std::io;

fn read_line() -> io::Result<String> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn is_prime(n: usize) -> bool {
    if n == 1 {
        return false;
    }
    let mut divisor = 2;
    while divisor*divisor <= n {
        if n % divisor == 0 {
            return false;
        }
        divisor += 1;
    }
    true
}


fn main() -> Result<(), Box<dyn error::Error>> {
    let t: usize = read_line()?.trim().parse()?;

    for _ in 0..t {
        let n: usize = read_line()?.trim().parse()?;
        if is_prime(n) {
            println!("Prime");
        } else {
            println!("Not prime");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(12), false);
        assert_eq!(is_prime(13), true);
    }
}