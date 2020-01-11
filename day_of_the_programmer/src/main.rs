use std::error;
use std::io;

fn read_line() -> Result<String, io::Error> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn is_leap_year_gregorian(year: usize) -> bool {
    if year % 4 != 0 {
        return false;
    }

    if year % 400 == 0 {
        return true;
    }

    if year % 100 == 0 {
        return false;
    }

    true
}

fn is_leap_year_julian(year: usize) -> bool {
    year % 4 == 0
}

fn solution(year: usize) -> String {
    if year > 1918 {
        if is_leap_year_gregorian(year) {
            return format!("12.09.{}", year);
        }

        return format!("13.09.{}", year);
    }

    if year == 1918 {
        return format!("26.09.{}", year);
    }

    if is_leap_year_julian(year) {
        return format!("12.09.{}", year);
    }

    return format!("13.09.{}", year);
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_line()?;
    let year: usize = input.trim().parse()?;
    let result = solution(year);
    println!("{}", result);

    Ok(())
}
