use std::collections::{HashMap, HashSet};
use std::error;
use std::io;
use std::iter::FromIterator;

fn read_line() -> Result<String, io::Error> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let n: usize = read_line()?.trim().parse()?;
    let mut occurencies = HashMap::new();

    for _ in 0..n {
        let input_chars: HashSet<char> = HashSet::from_iter(read_line()?.trim().chars());

        for ch in input_chars {
            *occurencies.entry(ch).or_insert(0) += 1;
        }
    }

    let result = occurencies.iter().filter(|(_, &v)| v==n).count();
    println!("{}", result);

    Ok(())
}
