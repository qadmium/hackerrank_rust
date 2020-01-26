use text_io::*;

fn solution(n: usize) -> usize {
    let mut recepients = 5;
    let mut result = 0;

    for _ in 1..=n {
        let likes = recepients / 2;
        result += likes;
        recepients = likes * 3;
    }

    result
}

fn main() {
    let result = solution(read!());
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        assert_eq!(solution(1), 2);
    }
    
    #[test]
    fn test_day_2() {
        assert_eq!(solution(2), 5);
    }
    
    #[test]
    fn test_day_3() {
        assert_eq!(solution(3), 9);
    }
}