use std::error::Error;
use std::io;
use std::str::FromStr;

fn read_line() -> Result<String, io::Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn read_array<T>() -> Result<Vec<T>, Box<dyn Error>> 
where
    T: FromStr,
    T::Err: Error + 'static,
{
    let result = read_line()?.trim().split(' ').map(&str::parse).collect::<Result<Vec<T>, _>>()?;
    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let t: usize = read_line()?.trim().parse()?;

    for _ in 0..t {
        let input = read_array()?;
        let n: usize = input[0];
        let k = input[1];

        let result = if k | (k - 1) <= n {
            k - 1
        } else {
            k - 2
        };

        println!("{}", result);
    }

    Ok(())
}