use text_io::*;

fn solution(d1: usize, m1: usize, y1: usize, d2: usize, m2: usize, y2: usize) -> usize {
    if y1 < y2 {
        return 0;
    }

    if y1 > y2 {
        return 10000;
    }

    if m1 < m2 {
        return 0;
    }

    if m1 > m2 {
        return (m1 - m2) * 500;
    }

    if d1 > d2 {
        return 15 * (d1 - d2);
    }

    0
}

fn main() {
    let result = solution(read!(), read!(), read!(), read!(), read!(), read!());
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::solution;

    #[test]
    fn test_sample_input() {
        assert_eq!(solution(9, 6, 2015, 6, 6, 2015), 45);
    }
}