use text_io::*;

fn solution(field: Vec<String>) -> Vec<String> {
    let field: Vec<Vec<_>> = field.into_iter().map(|row| row.bytes().collect()).collect();
    let mut result = field.clone();

    for i in 1..field.len() - 1 {
        for j in 1..field.len() - 1 {
            if field[i][j] > field[i][j - 1] &&
                field[i][j] > field[i][j + 1] && 
                field[i][j] > field[i - 1][j] &&
                field[i][j] > field[i + 1][j]
               {
                result[i][j] = b'X';
            }
        }
    }

    result.into_iter().map(|row| String::from_utf8(row).unwrap()).collect()
}

fn main() {
    let input = (0..read!()).map(|_| read!()).collect();
    let result = solution(input);

    for row in result {
        println!("{}", row);
    }
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_sample_input() {
        assert_eq!(
            solution(vec![
                "1112".to_owned(),
                "1912".to_owned(),
                "1892".to_owned(),
                "1234".to_owned(),
            ]),
            vec![
                "1112",
                "1X12",
                "18X2",
                "1234",
            ]
        )
    }   
}