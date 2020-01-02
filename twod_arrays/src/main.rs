use std::io;
use std::cmp::max;

const ARRAY_SIDE: usize = 6;
const SANDGLASS_SIDE: usize = 3;

fn sandglass_sum(a: &Vec<Vec<i64>>, i: usize, j: usize) -> i64 {
    a[i][j] + a[i][j+1] + a[i][j+2] +
    a[i+1][j+1] +
    a[i+2][j] + a[i+2][j+1] + a[i+2][j+2]
}

fn result(a: &Vec<Vec<i64>>) -> i64 {
    let mut result = -64;
    
    for i in 0..(ARRAY_SIDE-SANDGLASS_SIDE+1) {
        for j in 0..(ARRAY_SIDE-SANDGLASS_SIDE+1) {
            result = max(result, sandglass_sum(a, i, j));
        }
    }

    result
}

fn read_vec(input: &str) -> Vec<i64> {
    let vec: Vec<i64> = input.trim().split(" ").map(&str::trim)
        .map(&str::parse::<i64>).map(&Result::unwrap).collect();
    vec
}

fn main() {
    let mut a = Vec::new();
    for _ in 0..6 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("");
        a.push(read_vec(&input));
    }

    println!("{}", result(&a));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        let v = vec![
            vec![-1, -1, 0],
            vec![-2, -1, -6],
            vec![-1, -1, -1],
        ];
        assert_eq!(sandglass_sum(&v, 0, 0), -6);
    }

    #[test]
    fn test_read_vec() {
        let input = "-2 -1 -6 -8 -2 -5";
        assert_eq!(read_vec(input), vec![-2, -1, -6, -8, -2, -5]);
    }
}