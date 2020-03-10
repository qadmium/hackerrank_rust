use text_io::*;

fn front_and_back_surface(height: usize) -> usize {
    if height > 0 { 2 } else { 0 }
}

fn top_surface(i: usize, j: usize, toy: &[Vec<usize>]) -> usize {
    if i == 0 { toy[i][j] } else { toy[i][j].saturating_sub(toy[i - 1][j]) }
}

fn bottom_surface(i: usize, j: usize, toy: &[Vec<usize>]) -> usize {
    if i == toy.len() - 1 { toy[i][j] } else { toy[i][j].saturating_sub(toy[i + 1][j]) }
}

fn left_surface(i: usize, j: usize, toy: &[Vec<usize>]) -> usize {
    if j == 0 { toy[i][j] } else { toy[i][j].saturating_sub(toy[i][j - 1]) }
}

fn right_surface(i: usize, j: usize, toy: &[Vec<usize>]) -> usize {
    if j == toy[i].len() - 1 { toy[i][j] } else { toy[i][j].saturating_sub(toy[i][j + 1]) }
}

fn cell_surface(i: usize, j: usize, toy: &[Vec<usize>]) -> usize {
    front_and_back_surface(toy[i][j]) +
    top_surface(i, j, toy) +
    bottom_surface(i, j, toy) +
    left_surface(i, j, toy) +
    right_surface(i, j, toy)
}

fn solution(toy: Vec<Vec<usize>>) -> usize {
    let mut result = 0;
    for i in 0..toy.len() {
        for j in 0..toy[i].len() {
            result += cell_surface(i, j, &toy);
        }
    }

    result
}

fn main() {
    let h = read!();
    let w = read!();

    let input = (0..h)
        .map(|_| (0..w).map(|_| read!()).collect())
        .collect();
    let result = solution(input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_sample_input_0() {
        assert_eq!(solution(vec![vec![1]]), 6);
    }

    #[test]
    fn test_sample_input_1() {
        assert_eq!(solution(vec![
            vec![1, 3,  4],
            vec![2, 2, 3],
            vec![1, 2, 4],
        ]), 60);
    }
}