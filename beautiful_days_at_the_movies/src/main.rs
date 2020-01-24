use text_io::*;

fn reverse_number(mut n: usize) -> usize {
    let mut digits = Vec::new();
    while n > 0 {
        let remainder = n % 10;
        n = n / 10;
        digits.push(remainder);
    }

    digits.into_iter().fold(0, |acc, digit| acc * 10 + digit)
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn solution(i: usize, j: usize, k: usize) -> usize {
    (i..=j).filter(|&n| abs_diff(n, reverse_number(n)) % k == 0).count()
}

fn main() {
    let (i, j, k) = (read!(), read!(), read!());
    let result = solution(i, j, k);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        assert_eq!(solution(20, 23, 6), 2);
    }
}