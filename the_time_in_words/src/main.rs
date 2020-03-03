use text_io::*;

fn number_to_string(n: usize) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "tweleve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "quarter",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "half",
        _ => "",
    }
}

fn dozens_minutes(minutes: usize) -> &'static str {
    let minutes_to_render = if minutes > 30 {60 - minutes} else {minutes};
    if 20 < minutes_to_render && minutes_to_render < 30 {"twenty"} else {""}
}

fn hours_string(hours: usize, minutes: usize) -> &'static str {
    let hours_to_render = if minutes > 30 {hours + 1} else {hours};
    number_to_string(hours_to_render)
}

fn past_or_two(minutes: usize) -> &'static str {
    match minutes {
        0 => "",
        1..=30 => "past",
        _ => "to",
    }
}

fn oclock(minutes: usize) -> &'static str {
    if minutes == 0 {"o' clock"} else {""}
}

fn minute_word(minutes: usize) -> &'static str {
    let minutes_to_render = if minutes > 30 {60 - minutes} else {minutes};
    match minutes_to_render {
        1 => "minute",
        0 | 15 | 30 => "",
        _ => "minutes",
    }
}

fn minutes_string(minutes: usize) -> &'static str {
    let minutes_to_render = if minutes > 30 {60 - minutes} else {minutes};
    match minutes_to_render {
        0 => "",
        n if 20 < n && n < 30 => number_to_string(minutes_to_render % 20),
        _ => number_to_string(minutes_to_render),
    }
}

fn solution(hours_str: String, minutes_str: String) -> String {
    let hours: usize = hours_str.parse().unwrap();
    let minutes: usize = minutes_str.parse().unwrap();

    let to_format = vec![
        dozens_minutes(minutes),
        minutes_string(minutes),
        minute_word(minutes),
        past_or_two(minutes),
        hours_string(hours, minutes),
        oclock(minutes)
    ];

    let to_format: Vec<_> = to_format.into_iter().filter(|s| !s.is_empty()).collect();
    to_format.join(" ")
}

fn main() {
    let result = solution(read!(), read!());
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_sample_input() {
        assert_eq!(solution("5".to_owned(), "00".to_owned()), "five o' clock");
        assert_eq!(solution("5".to_owned(), "01".to_owned()), "one minute past five");
        assert_eq!(solution("5".to_owned(), "10".to_owned()), "ten minutes past five");
        assert_eq!(solution("5".to_owned(), "15".to_owned()), "quarter past five");
        assert_eq!(solution("5".to_owned(), "30".to_owned()), "half past five");
        assert_eq!(solution("5".to_owned(), "40".to_owned()), "twenty minutes to six");
        assert_eq!(solution("5".to_owned(), "45".to_owned()), "quarter to six");
        assert_eq!(solution("5".to_owned(), "47".to_owned()), "thirteen minutes to six");
        assert_eq!(solution("5".to_owned(), "28".to_owned()), "twenty eight minutes past five");
    }

    #[test]
    fn test_case_6() {
        assert_eq!(solution("6".to_owned(), "35".to_owned()), "twenty five minutes to seven")
    }
}