use text_io::*;

struct SolutionState {
    v: Vec<usize>,
    x: usize,
}

impl Iterator for SolutionState {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.x += 1;
        if self.x > self.v.len() {
            return None;
        }

        let (idx, _) = self.v.iter().enumerate().find(|(_, &n)| n == self.x).unwrap();
        let (result, _) = self.v.iter().enumerate().find(|(_, &n)| n == idx + 1).unwrap();
        Some(result + 1)
    }
}

fn solution(v: Vec<usize>) -> SolutionState {
    SolutionState{v, x: 0}
}

fn main() {
    let n = read!();
    let vec = (0..n).map(|_| read!()).collect();

    for item in solution(vec) {
        println!("{}", item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(solution(vec![5, 2, 1, 3, 4]).collect::<Vec<_>>(), vec![4, 2, 5, 1, 3]);
    }
}
