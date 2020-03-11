use std::fmt;
use num::Integer;
use text_io::*;

struct Board {
    _board: Vec<Vec<Option<u8>>>,
}

impl Board {
    fn from_input(init: Vec<Vec<u8>>) -> Board {
        let mut state = Vec::with_capacity(init.len());

        for i in 0..init.len() {
            state.push(Vec::new());
            for j in 0..init[i].len() {
                let point = init[i][j];
                let cell = if point == b'.' { None } else { Some(2) };
                state[i].push(cell);
            }
        }

        Board {_board: state }
    }

    fn _update<F>(&mut self, f: F)
        where F: Fn(Option<u8>) -> Option<u8> 
    {
        for i in 0..self._board.len() {
            for j in 0..self._board[i].len() {
                self._board[i][j] = f(self._board[i][j]);
            }
        }
    }

    fn _tick(&mut self) {
        self._update(|cell| cell.map(|time| time - 1));
    }
    
    fn _fill(&mut self) {
        self._update(|cell| cell.or(Some(3)));
    }

    fn _can_clean_top(&self, i: usize, j: usize) -> bool {
        i != 0 && self._board[i - 1][j].map_or(false, |time| time > 0)
    }

    fn _can_clean_bottom(&self, i: usize, j: usize) -> bool {
        i != self._board.len() - 1 && self._board[i + 1][j].map_or(false, |time| time > 0)
    }

    fn _can_clean_left(&self, i: usize, j: usize) -> bool {
        j != 0 && self._board[i][j - 1].map_or(false, |time| time > 0)
    }

    fn _can_clean_right(&self, i: usize, j: usize) -> bool {
        j != self._board[i].len() - 1 && self._board[i][j + 1].map_or(false, |time| time > 0)
    }

    fn _explode(&mut self) {
        for i in 0..self._board.len() {
            for j in 0..self._board[i].len() {
                if !self._board[i][j].map_or(false, |time| time == 0) {
                    continue;
                }

                self._board[i][j] = None;

                if self._can_clean_top(i, j) {
                    self._board[i - 1][j] = None;
                }

                if self._can_clean_bottom(i, j) {
                    self._board[i + 1][j] = None;
                }

                if self._can_clean_left(i, j) {
                    self._board[i][j - 1] = None;
                }
               
                if self._can_clean_right(i, j) {
                    self._board[i][j + 1] = None;
                }
            }
        }
    }

    fn step(&mut self) {
        self._tick();
        self._fill();
        self._explode();
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self._board.len() {
            if i != 0 {
                writeln!(f)?;
            }

            for j in 0..self._board[i].len() {
                write!(f, "{}", self._board[i][j].map_or('.', |_| 'O'))?;
            }
        }

        Ok(())
    }
} 

fn bomber_game(init: Vec<Vec<u8>>, n: usize) -> Board {
    let mut board = Board::from_input(init);

    for _ in 0..n - 1 {
        board.step();
    }

    board
}

fn solution(init: Vec<Vec<u8>>, n: usize) -> Board {
    if n == 1 {
        return bomber_game(init, 1);
    }

    if n.is_even() {
        return bomber_game(init, 2);
    }

    if (n - 1) % 4 == 0{
        return bomber_game(init, 5);
    }

    bomber_game(init, 3)
}

fn main() {
    let r = read!();
    let _: usize = read!();
    let n = read!();
    let input = (0..r)
        .map(|_| {
            let row: String = read!();
            row.bytes().collect()
        })
        .collect();
    let result = solution(input, n);
    println!("{}", result);
}
