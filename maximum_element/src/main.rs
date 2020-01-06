use std::collections::VecDeque;
use std::error;
use std::io;

fn read_line() -> Result<String, io::Error> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let n: usize = read_line()?.trim().parse()?;
    let mut stack = VecDeque::new();

    for _ in 0..n {
        let input = read_line()?;
        let mut iter = input.trim().split(' ').map(&str::parse::<u32>);

        match iter.next() {
            Some(Ok(1)) => match iter.next() {
                Some(Ok(number)) => stack.push_back(number),
                _ => panic!("not expected input"),
            },
            Some(Ok(2)) => { stack.pop_back(); },
            Some(Ok(3)) => println!("{}", stack.iter().max().expect("stack is empty")),
            _ => panic!("not expected input"),
        };
    }

    Ok(())
}
