use std::error;
use std::io;

fn read_line() -> Result<String, io::Error> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

const ASCII_SIZE: usize = (b'z' - b'a' + 1) as usize;
type CharsState = [usize; ASCII_SIZE];

#[derive(Clone)]
struct MatchState {
    string: Vec<u8>,
    chars_left_in_string: Vec<CharsState>,
    chars_to_consume: CharsState, 
    lower_available_char: Option<u8>,
    idx: usize,
    skipped: Vec<u8>,
}

impl MatchState {
    fn offset(ch: u8) -> usize {
        (ch - b'a') as usize
    }

    fn can_advance(&self) -> bool {
        self.idx < self.string.len()
    }

    fn lower_available_char(s: &CharsState) -> Option<u8> {
        let result = s.iter().enumerate().find(|(_, &v)| v != 0);
        result.map(|(idx, _)| idx as u8 + b'a')
    }

    fn from_str(s: &str) -> MatchState {
        let mut chars_left_in_string = Vec::with_capacity(s.len());
        let mut chars_to_consume = [0; ASCII_SIZE];
        
        for ch in s.bytes() {
            let offset = Self::offset(ch);
            chars_to_consume[offset] += 1;

            let mut chars_left_for_this_position = [0; ASCII_SIZE];
            chars_left_for_this_position.clone_from_slice(&chars_to_consume);

            chars_left_in_string.push(chars_left_for_this_position);
        }

        chars_left_in_string.reverse();

        for to_consume in chars_to_consume.iter_mut().take(ASCII_SIZE) {
            *to_consume /= 2;
        }

        MatchState {
            string: s.bytes().rev().collect(),
            chars_left_in_string, 
            chars_to_consume,
            lower_available_char: Self::lower_available_char(&chars_to_consume),
            idx: 0,
            skipped: Vec::new(),
        }
    }

    fn is_lower_possible_value(&self) -> bool {
        self.lower_available_char.map_or(false, |v| v == self.current_ch())
    }

    fn can_skip(&self) -> bool {
        if self.is_lower_possible_value() {
            return false;
        }
        
        let offset = Self::offset(self.current_ch());
        let left_to_consume_count = self.chars_to_consume[offset];
        if left_to_consume_count == 0 {
            return true;
        }
        let left_in_string_count = self.chars_left_in_string[self.idx][offset];
        left_to_consume_count < left_in_string_count
    }

    fn current_ch(&self) -> u8 {
        self.string[self.idx]
    }

    fn eat(&mut self) {
        let left_to_eat = &mut self.chars_to_consume[Self::offset(self.current_ch())];

        *left_to_eat -= 1;
        if *left_to_eat == 0 {
            self.lower_available_char = Self::lower_available_char(&self.chars_to_consume);
        }

        self.skipped.clear();
        self.idx += 1;
    }

    fn skip(&mut self) {
        self.skipped.push(self.current_ch());
        self.idx += 1;
    }

    fn chars_to_eat<'a>(&'a self) -> impl Iterator<Item = u8> + 'a {
        self.chars_to_consume.iter().enumerate().filter(|(_, &v)| v != 0).map(|(idx, _)| idx as u8 + b'a')
    }

    fn try_rollback(&mut self) {
        self.skipped.push(self.current_ch());

        let (idx, _) = self.chars_to_eat().filter_map(
            |ch| self.skipped.iter().enumerate().find(|(_, &skipped)| skipped == ch)
        ).next().expect("we have at least one char in skipped queue");

        self.idx -= self.skipped.len() - idx - 1
    }
}

fn solution(s: &str) -> String {
    assert!(s.is_ascii());
    let mut state = MatchState::from_str(s);
    let mut result = Vec::new();

    while state.can_advance() {
        if state.can_skip() {
            state.skip();
        } else if state.is_lower_possible_value() {
            result.push(state.current_ch());
            state.eat();
        } else {
            state.try_rollback();
            result.push(state.current_ch());
            state.eat();
        }

    }
    String::from_utf8(result.into_iter().collect()).unwrap()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_line()?;
    let result = solution(input.trim());
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_input_0() {
        assert_eq!(solution("eggegg"), "egg");
    }
    
    #[test]
    fn test_sample_input_1() {
        assert_eq!(solution("abcdefgabcdefg"), "agfedcb");
    }

    #[test]
    fn test_sample_input_2() {
        assert_eq!(solution("aeiouuoiea"), "aeiou");
    }

    #[test]
    fn test_case_1() {
        assert_eq!(solution("bdabaceadaedaaaeaecdeadababdbeaeeacacaba"), "aaaaaabaaceededecbdb");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(solution(
            "djjcddjggbiigjhfghehhbgdigjicafgjcehhfgifadihiajgciagicdahcbajjbhifjiaajigdgdfhdiijjgaiejgegbbiigida"), 
            "aaaaabccigicgjihidfiejfijgidgbhhehgfhjgiibggjddjjd"
        );
    }

}