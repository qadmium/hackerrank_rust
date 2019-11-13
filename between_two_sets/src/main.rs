use std::io;
use std::collections::HashSet;

fn divisors(n: usize) -> HashSet<usize> {
    let mut result = HashSet::new();

    let mut divisor = 1;
    while divisor*divisor <= n {
        if n % divisor == 0 {
            result.insert(divisor);
            result.insert(n / divisor);
        }
        divisor += 1;
    }
    result
}

fn check_divisor(first_set: &HashSet<usize>, second_set: &HashSet<usize>, divisor: usize) -> bool {
    first_set.iter().all(|factor| divisor % factor == 0) && second_set.iter().all(|item| item % divisor == 0)
}

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("cant read");
    input.clear();

    io::stdin().read_line(&mut input).expect("cant read");

    let first_set: HashSet<usize> = input.trim().split(" ").map(&str::trim)
        .map(&str::parse::<usize>).map(|n| n.expect("int expected")).collect();

    input.clear();

    io::stdin().read_line(&mut input).expect("cant read");
    let second_set: HashSet<usize> = input.trim().split(" ").map(&str::trim)
        .map(&str::parse::<usize>).map(|n| n.expect("int expected")).collect();

    if second_set.is_empty() {
        println!("0");
        return;
    }
    
    let min_value_divisors = divisors(*second_set.iter().min().expect("should not be empty"));
    let result = min_value_divisors.iter().filter(|&item| check_divisor(&first_set, &second_set, *item)).count();
    println!("{}", result);
}