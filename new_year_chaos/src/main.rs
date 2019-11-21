use std::io;

fn calc_bribes(vec: &mut Vec<usize>) -> Option<usize> {
    if vec.len() == 0 {
        return None;
    }
    let mut result = 0;
    let mut i = vec.len() - 1;

    loop {
        let expected_item = i + 1;
        let current_item = vec[i];

        if current_item <= expected_item {
            if i == 0 {
                return Some(result);
            }

            i -= 1;
        } else {
            let distance = current_item - expected_item;
            assert!(distance > 0);

            if distance > 2 {
                return None;
            }

            &vec[i..i+1+distance].rotate_left(1);
            result += distance;
        }
    }
}

fn main() {
    let mut n_test_cases_string = String::new();
    io::stdin().read_line(&mut n_test_cases_string).expect("");
    let n: usize = n_test_cases_string.trim().parse().expect("");

    for _ in 0..n {
        let mut size_string = String::new();
        io::stdin().read_line(&mut size_string).expect("");
        let mut arr = String::new();
        io::stdin().read_line(&mut arr).expect("");

        let mut vec = arr.trim().split(" ").map(&str::trim).map(&str::parse::<usize>)
            .collect::<Result<Vec<usize>, _>>().expect("some err while parsing");

        match calc_bribes(&mut vec) {
            Some(n) => println!("{}", n),
            None => println!("Too chaotic"),
        }
    }
}
