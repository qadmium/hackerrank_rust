use text_io::*;

fn solution(s: String) -> Option<String> {
    let mut result: Vec<_> = s.chars().collect();

    let (idx_to_exchange, ch_to_exchange) = result
        .windows(2)
        .enumerate()
        .map(|(window_idx, window)| ((window_idx, window[0]), (window_idx+1, window[1])))
        .rev()
        .find(|((_, left_ch), (_, right_ch))| left_ch < right_ch)?
        .0;

    let right_idx_to_exchange = idx_to_exchange + 1 + result[idx_to_exchange+1..]
        .iter()
        .enumerate()
        .filter(|(_, &ch)| ch > ch_to_exchange)
        .min_by(|(_, left_ch), (_, right_ch)| left_ch.cmp(right_ch))?
        .0;

    result.swap(idx_to_exchange, right_idx_to_exchange);
    result[idx_to_exchange+1..].sort();

    Some(result.into_iter().collect())
}

fn main() {
    for _ in 0..read!() {
        let result = solution(read!());
        println!("{}", result.unwrap_or_else(|| "no answer".to_owned()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        assert_eq!(solution("ab".to_owned()), Some("ba".to_owned()));
        assert_eq!(solution("bb".to_owned()), None);
        assert_eq!(solution("hefg".to_owned()), Some("hegf".to_owned()));
        assert_eq!(solution("dhck".to_owned()), Some("dhkc".to_owned()));
        assert_eq!(solution("bdca".to_owned()), Some("cabd".to_owned()));
        assert_eq!(solution("dkhc".to_owned()), Some("hcdk".to_owned()));
    }
}
