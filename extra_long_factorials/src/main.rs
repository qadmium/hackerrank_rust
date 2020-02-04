use num::{BigUint, One};
use text_io::*;

fn main() {
    let n: usize = read!();

    let mut result: BigUint = One::one();

    for i in 1..=n {
        result *= i;
    }
    println!("{}", result);
}
