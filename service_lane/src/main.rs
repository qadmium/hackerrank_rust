use text_io::*;

fn solution(segments: &[usize], i: usize, j: usize) -> &usize {
    segments[i..=j].iter().min().expect("segments sould not be empty")
}

fn main() {
    let n = read!();
    let t = read!();
    let segments: Vec<_> = (0..n).map(|_| read!()).collect();

    for _ in 0..t {
        let result = solution(&segments, read!(), read!());
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let segments = vec![2, 3, 1, 2, 3, 2, 3, 3];
        assert_eq!(*solution(&segments, 0, 3), 1);
        assert_eq!(*solution(&segments, 4, 6), 2);
        assert_eq!(*solution(&segments, 6, 7), 3);
        assert_eq!(*solution(&segments, 3, 5), 2);
        assert_eq!(*solution(&segments, 0, 7), 1);
    }
}