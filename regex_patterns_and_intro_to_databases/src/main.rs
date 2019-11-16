extern crate regex;

use std::io;
use std::error::Error;
use regex::Regex;

fn read_line() -> Result<String, io::Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn read_n_lines(n: usize) -> Result<Vec<String>, io::Error> {
    let mut result = Vec::new();
    for _ in 0..n {
        result.push(read_line()?);
    }
    Ok(result)
}

fn find_names<'a>(table: &'a [&str], regex: &Regex) -> Vec<&'a str> {
    let mut result: Vec<_> = table
        .iter()
        .filter_map(|line| regex.captures(line))
        .filter_map(|cap| cap.name("name").map(|name| name.as_str()))
        .collect();

    result.sort();
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let n: usize = read_line()?.trim().parse()?;
    let regex = Regex::new(r"(?P<name>[a-z]*) [a-z\.]*@gmail.com")?;
    let table = read_n_lines(n)?;
    let tmp: Vec<&str> = table.iter().map(AsRef::as_ref).collect();

    for item in find_names(&tmp, &regex).iter() {
        println!("{}", item);
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_gmail() {
        let regex = Regex::new(r"(?P<name>[a-z]*) [a-z\.]*@gmail.com").unwrap();
        let input = vec!["qwe qwr@gmail.com", "asd asfd@sdfd.ru", "aaa aaa@gmail.com"];
        assert_eq!(find_names(&input, &regex), vec!["aaa", "qwe"]);
    }

    #[test]
    fn test_case_2() {
        let regex = Regex::new(r"(?P<name>[a-z]*) [a-z\.]*@gmail.com").unwrap();
        let input = vec!["asd as.fd@gmail.com"];
        assert_eq!(find_names(&input, &regex), vec!["asd"]);
    }
}