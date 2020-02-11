use text_io::*;

fn solution(money: usize, cost: usize, exchange_rate: usize) -> usize {
    let mut bars = money / cost;
    let mut wrappers = bars;
    while wrappers >= exchange_rate {
        let bars_got = wrappers / exchange_rate;
        bars += bars_got;
        let wrappers_left = wrappers % exchange_rate;
        wrappers = wrappers_left + bars_got;
    }
    bars
}

fn main() {
    for _ in 0..read!() {
        let result = solution(read!(), read!(), read!()); 
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_sample_input() {
        assert_eq!(solution(10, 2, 5), 6);
        assert_eq!(solution(12, 4, 4), 3);
        assert_eq!(solution(6, 2, 2), 5);
    }
}