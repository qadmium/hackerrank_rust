use std::error;
use std::io;

fn read_line() -> Result<String, io::Error> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

fn solution(mut arr: Vec<usize>, k: usize) -> usize {
    let slice_unfairness = |slice: &[usize]| slice[slice.len() - 1] - slice[0];
    arr.sort();
    arr.windows(k)
        .min_by(|left, right| slice_unfairness(left).cmp(&slice_unfairness(right)))
        .map_or(0, |slice| slice_unfairness(slice))
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let n: usize = read_line()?.trim().parse()?;
    let k: usize = read_line()?.trim().parse()?;

    let mut arr = Vec::new();

    for _ in 0..n {
        let item: usize = read_line()?.trim().parse()?;
        arr.push(item);
    }

    let result = solution(arr, k);
    println!("{}", result);

    Ok(())
}
