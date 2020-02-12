
use std::collections::HashSet;
use text_io::*;

fn binary_string_to_topics_set(s: String) -> HashSet<usize> {
    assert!(s.is_ascii() && s.bytes().all(|b| b == b'0' || b == b'1'));

    s.bytes()
        .enumerate()
        .filter(|(_, value)| *value == b'1')
        .map(|(idx, _)| idx)
        .collect()
}

fn solution(members: Vec<HashSet<usize>>) -> (usize, usize) {
    let mut max_topics = 0;
    let mut teams = 0;
    for i in 0..members.len() - 1 {
        for j in i+1..members.len() {
            let team_topics = members[i].union(&members[j]).count();
            if team_topics > max_topics {
                max_topics = team_topics;
                teams = 1;
            } else if team_topics == max_topics {
                teams += 1;
            }
        }
    }

    (max_topics, teams)
}

fn main() {
    let n = read!();
    let _: usize = read!();
    let members = (0..n).map(|_| binary_string_to_topics_set(read!())).collect();
    let (max_topics, teams) = solution(members);
    println!("{}", max_topics);
    println!("{}", teams);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(binary_string_to_topics_set("01010101".to_owned()), vec![1, 3 , 5, 7].into_iter().collect());
    }


    #[test]
    fn test_sample_input() {
        let input = vec![
            binary_string_to_topics_set("10101".to_owned()),
            binary_string_to_topics_set("11100".to_owned()),
            binary_string_to_topics_set("11010".to_owned()),
            binary_string_to_topics_set("00101".to_owned()),
        ];
        assert_eq!(solution(input), (5, 2));
    }
}
