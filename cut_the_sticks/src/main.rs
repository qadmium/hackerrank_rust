use text_io::*;

struct State {
    v: Vec<usize>,
}

impl Iterator for State {
    type Item=usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.v.len() {
            0 => None,
            len => {
                let min = self.v.iter().min()?;
                self.v = self.v.iter().map(|item| item - min).filter(|&item| item > 0).collect();
                Some(len)
            }
        }
    }
}

fn solution(v: Vec<usize>) -> impl Iterator<Item=usize> {
    State{v}
}

fn main() {
    let input: Vec<_> = (0..read!()).map(|_| read!()).collect();
    let result = solution(input);

    for item in result {
        println!("{}", item);
    }
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_sample_input() {
        assert_eq!(solution(vec![1, 2, 3, 4, 3, 3, 2, 1]).collect::<Vec<_>>(), vec![8, 6, 4, 1]);
    }
}