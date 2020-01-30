use text_io::*;

fn solution(from: usize, to: usize) -> usize {
    let start_from = (from as f64).sqrt() as usize;
    (start_from..)
        .skip_while(|i| i*i < from)
        .take_while(|i| i*i <= to)
        .count()
}

fn main() {
    for _ in 0..read!() {
        let result = solution(read!(), read!());
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_sample_input_0() {
        assert_eq!(solution(3, 9), 2);
        assert_eq!(solution(17, 24), 0);
    }
}
