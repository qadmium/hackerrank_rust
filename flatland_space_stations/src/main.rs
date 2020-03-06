use text_io::*;

fn solution(mut v: Vec<usize>, n: usize) -> usize {
    v.sort();

    let mut distances: Vec<_> = v
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .filter(|&distance| distance > 1)
        .map(|distance| distance / 2)
        .collect();
    let first_distance = v[0] - 0;
    let last_distance = n - 1 - v[v.len() - 1];
    distances.push(first_distance);
    distances.push(last_distance);
    distances.into_iter().max().unwrap() 
}

fn main() {
    let n = read!();
    let input = (0..read!()).map(|_| read!()).collect();
    let result = solution(input, n);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_space_station() {
        assert_eq!(solution(vec![0], 3), 2);
        assert_eq!(solution(vec![2], 3), 2);
        assert_eq!(solution(vec![1], 3), 1);
    }

    #[test]
    fn test_odd_middle() {
        assert_eq!(solution(vec![0, 5], 6), 2);
    }

    #[test]
    fn test_sample_input_0() {
        assert_eq!(solution(vec![0, 4], 5), 2);
    }

    #[test]
    fn test_sample_input_1() {
        assert_eq!(solution(vec![0, 1, 2, 3, 4, 5], 6), 0);
    }
}