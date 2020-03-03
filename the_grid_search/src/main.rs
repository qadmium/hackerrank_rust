use text_io::*;

fn matches(grid: &Vec<Vec<u8>>, pat: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
    for pat_i in 0..pat.len() {
        for pat_j in 0..pat[pat_i].len() {
            if pat[pat_i][pat_j] != grid[i + pat_i][j + pat_j] {
                return false;
            }
        }
    }

    true
}

fn solution(grid: Vec<Vec<u8>>, pat: Vec<Vec<u8>>) -> bool {
    for i in 0..grid.len() - (pat.len() - 1) {
        for j in 0..grid[i].len() - (pat[0].len() - 1) {
            if matches(&grid, &pat, i, j) {
                return true;
            }
        }
    }

    false
}

fn main() {
    for _ in 0..read!() {
        let rows = read!();
        let _: usize = read!();
        let grid = (0..rows).map(|_| {
            let s: String = read!();
            s.bytes().collect()
        }).collect();

        let rows = read!();
        let _: usize = read!();
        let pat = (0..rows).map(|_| {
            let s: String = read!();
            s.bytes().collect()
        }).collect();
        let result = solution(grid, pat);
        println!("{}", if result {"YES"} else {"NO"})
    }
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_sample_input_0() {
        assert_eq!(
            solution(vec![
                vec![7, 2, 8, 3, 4, 5, 5, 8, 6, 4],
                vec![6, 7, 3, 1, 1, 5, 8, 6, 1, 9],
                vec![8, 9, 8, 8, 2, 4, 2, 6, 4, 3],
                vec![3, 8, 3, 0, 5, 8, 9, 3, 2, 4],
                vec![2, 2, 2, 9, 5, 0, 5, 8, 1, 3],
                vec![5, 6, 3, 3, 8, 4, 5, 3, 7, 4],
                vec![6, 4, 7, 3, 5, 3, 0, 2, 9, 3],
                vec![7, 0, 5, 3, 1, 0, 6, 6, 0, 1],
                vec![0, 8, 3, 4, 2, 8, 2, 9, 5, 6],
                vec![4, 6, 0, 7, 9, 2, 4, 1, 3, 7]

            ], 
            vec![
                vec![9, 5, 0, 5],
                vec![3, 8, 4, 5],
                vec![3, 5, 3, 0],
            ]),
            true
        );
    }   
}