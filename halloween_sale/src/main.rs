use text_io::*;

fn solution(price: usize, discount : usize, min: usize, cash: usize) -> usize {
    let mut result = 0;
    let mut next_buy_price = price;
    let mut cash_left = cash;

    while next_buy_price > min {
        if cash_left < next_buy_price {
            return result;
        }
        
        cash_left -= next_buy_price;
        next_buy_price = next_buy_price.saturating_sub(discount);
        result += 1; 
    }
    
    result += cash_left / min;
    result
}

fn main() {
    let result = solution(read!(), read!(), read!(), read!());
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_sample_input_0() {
        assert_eq!(solution(20, 3, 6, 80), 6);
    }

    #[test]
    fn test_sample_input_1() {
        assert_eq!(solution(20, 3, 6, 85), 7);
    }

    #[test]
    fn test_not_enough_cash() {
        assert_eq!(solution(20, 3, 6, 1), 0);
        assert_eq!(solution(100, 1, 1, 99), 0);
    }
    
    #[test]
    fn test_discount_greater_than_price() {
        assert_eq!(solution(20, 30, 6, 26), 2);
    }
}
