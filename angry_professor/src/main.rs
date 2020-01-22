use text_io::*;

fn solution(v: Vec<i64>, k: usize) -> bool {
    let arrived_students = v.into_iter().filter(|&i| i <= 0).count();
    arrived_students < k
}

fn main() {
    let input_size: usize = read!();
    for _ in 0..input_size {
        let (n, k) = (read!(), read!());
        let v = (0..n).map(|_| read!()).collect();
        let result = solution(v, k);
        println!("{}", if result {"YES"} else {"NO"});
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        assert_eq!(solution(vec![-1, -3, 4, 2], 3), true);
        assert_eq!(solution(vec![0, -1, 2, 1], 2), false);
    }
}
