use text_io::*;

fn solution(n: usize) -> String {
    let mut i = n;
    
    loop {
        if i % 3 == 0 && (n - i) % 5 == 0 {
            return format!("{}{}", "5".repeat(i), "3".repeat(n - i))
        }

        if i < 5 {
            return "-1".to_owned()
        }

        i -= 5;
    }
}

fn main() {
    for _ in 0..read!() {
        let result = solution(read!());
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_0() {
        assert_eq!(solution(1), "-1");
        assert_eq!(solution(3), "555");
        assert_eq!(solution(5), "33333");
        assert_eq!(solution(11), "55555533333");
    }
}