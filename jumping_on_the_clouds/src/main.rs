use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("some err");
    input.clear();

    io::stdin().read_line(&mut input).expect("some err");
    let clouds = input.trim().split(" ").map(&str::trim).map(&str::parse::<usize>)
        .collect::<Result<Vec<usize>, _>>().expect("some err while parsing");

    let mut idx = 0;
    let mut result = 0;
    let last_idx = clouds.len() - 1;
    while idx < last_idx {
        let jump_distance = if idx + 2 > last_idx || clouds[idx + 2] == 1 {
            1
        } else {
            2
        };

        idx += jump_distance;
        result += 1;
    }

    println!("{}", result);
}
