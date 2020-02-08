use text_io::*;

fn solution(v: Vec<usize>) -> usize {
    v.into_iter()
        .rev()
        .fold(0, |need_energy, height| (need_energy + height + 1) / 2)
}

fn main() {
    let input = (0..read!()).map(|_| read!()).collect();
    let result = solution(input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test() {
        assert_eq!(solution(vec![6]), 3);
        assert_eq!(solution(vec![1, 6]), 2);
        assert_eq!(solution(vec![3, 4]), 3);
    }  

    #[test]
    fn test_sample_input_0() {
        assert_eq!(solution(vec![3, 4, 3, 2, 4]), 4);
    }

    #[test]
    fn test_sample_input_1() {
        assert_eq!(solution(vec![4, 4, 4]), 4);
    }

    #[test]
    fn test_sample_input_2() {
        assert_eq!(solution(vec![1, 6, 4]), 3);
    }  
}