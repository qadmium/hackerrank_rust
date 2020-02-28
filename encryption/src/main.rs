use num::integer::Roots;
use text_io::*;

fn calc_size(s_len: usize) -> (usize, usize) {
    let rows = s_len.sqrt();
    let columns = if rows*rows < s_len {
        rows + 1
    } else {
        rows
    };

    if rows*columns < s_len {
        (rows + 1, columns)
    } else {
        (rows, columns)
    }
}

fn solution(mut s: String) -> String {
    assert!(s.is_ascii());
    s = s.replace(' ', "");

    let (rows, columns) = calc_size(s.len());
    let mut arr = vec![vec![0; columns]; rows];

    for (idx, ch) in s.bytes().enumerate() {
        let column = idx % columns;
        let row = idx / columns;
        arr[row][column] = ch;
    }

    let mut result = Vec::new();

    for i in 0..columns {
        let mut j = 0;
        while j < rows && arr[j][i] != 0 {
            result.push(arr[j][i]);
            j += 1;
        }

        result.push(b' ');
    }
    result.remove(result.len() - 1);
    String::from_utf8(result).unwrap()
}

fn main() {
    let result = solution(read!());
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_sample_input() {
        assert_eq!(
            solution("if man was meant to stay on the ground god would have given us roots".to_owned()),
            "imtgdvs fearwer mayoogo anouuio ntnnlvt wttddes aohghn sseoau"
        );

        assert_eq!(
            solution("haveaniceday".to_owned()),
            "hae and via ecy"
        );
    }

    #[test]
    fn test_sample_test_case_2() {
        assert_eq!(
            solution("chillout".to_owned()),
            "clu hlt io",
        )
    }
}