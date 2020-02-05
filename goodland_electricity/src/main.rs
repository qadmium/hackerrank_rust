use std::cmp::min;
use text_io::*;

fn solution(cities: Vec<u8>, k: usize) -> Option<usize> {
    let mut result = 0;
    let mut city_to_connect_idx = 0;

    while city_to_connect_idx < cities.len() {
        let furthest_possible_place = min(city_to_connect_idx + k, cities.len());
        let closest_possible_place = match city_to_connect_idx.overflowing_sub(k) {
            (val, false) => val + 1,
            (_, true) => 0,
        };

        let plant_range = &cities[closest_possible_place..furthest_possible_place];
        let found_place_for_plant = plant_range.iter().rev().enumerate().find(|(_, &val)| val != 0);

        let idx_for_plant = found_place_for_plant.map(|(idx, _)| idx).map(|idx| furthest_possible_place - 1 - idx)?;
        result += 1;

        city_to_connect_idx = idx_for_plant + k;
    }

    Some(result)
}

fn main() {
    let n = read!();
    let k = read!();
    let cities: Vec<_> = (0..n).map(|_| read!()).collect();
    let result = solution(cities, k).map(|result| result as i64).unwrap_or(-1);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_empty() {
        assert_eq!(solution(vec![0], 5), None);
        assert_eq!(solution(vec![0, 0, 0, 0, 0, 0], 5), None);
    }

    #[test]
    fn test_one() {
        assert_eq!(solution(vec![1, 1, 1], 2), Some(1));
        assert_eq!(solution(vec![1, 1, 1, 1, 1], 3), Some(1));
    }

    #[test]
    fn test_place_plant_before() {
        assert_eq!(solution(vec![1, 1, 0, 1, 0], 3), Some(2));
    }

    #[test]
    fn test_sample_input() {
        assert_eq!(solution(vec![0, 1, 1, 1, 1, 0], 2), Some(2));
    }
}