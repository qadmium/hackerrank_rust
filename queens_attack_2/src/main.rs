use text_io::*;

#[derive(Debug)]
struct Point {
    row: usize,
    column: usize,
}

impl Point {
    fn new(row: usize, column: usize) -> Point {
        Point{row, column}
    }
}

struct State {
    size: usize,
    queen: Point,
    obstacles: Vec<Point>,
}

fn row_left_available(state: &State) -> usize {
    let closest_obstacle_column = state.obstacles
        .iter()
        .filter(|p| p.row == state.queen.row && p.column < state.queen.column)
        .map(|p| p.column)
        .max()
        .unwrap_or_default();
    state.queen.column - closest_obstacle_column - 1
}

fn row_right_available(state: &State) -> usize {
    let closest_obstacle_column = state.obstacles
        .iter()
        .filter(|p| p.row == state.queen.row && p.column > state.queen.column)
        .map(|p| p.column)
        .min()
        .unwrap_or(state.size + 1);
    closest_obstacle_column - state.queen.column - 1
}

fn column_up_available(state: &State) -> usize {
    let closest_obstacle_row = state.obstacles
        .iter()
        .filter(|p| p.column == state.queen.column && p.row < state.queen.row)
        .map(|p| p.row)
        .max()
        .unwrap_or_default();
    state.queen.row - closest_obstacle_row - 1
}

fn column_down_available(state: &State) -> usize {
    let closest_obstacle_row = state.obstacles
        .iter()
        .filter(|p| p.column == state.queen.column && p.row > state.queen.row)
        .map(|p| p.row)
        .min()
        .unwrap_or(state.size + 1);
    closest_obstacle_row - state.queen.row - 1
}

fn diag1_up_available(state: &State) -> usize {
    let diag1_up_key = state.size + state.queen.row - state.queen.column;
    let default = if state.queen.row > state.queen.column { 0 } else { state.queen.column - state.queen.row };
    let closest_obstacle_column = state.obstacles
        .iter()
        .filter(|p| p.column < state.queen.column && p.row < state.queen.row)
        .filter(|p| state.size + p.row - p.column == diag1_up_key)
        .map(|p| p.column)
        .max()
        .unwrap_or(default);

    state.queen.column - closest_obstacle_column - 1
}

fn diag1_down_available(state: &State) -> usize {
    let diag1_down_diff = state.size + state.queen.row - state.queen.column;
    let default = if state.queen.row > state.queen.column { 
        state.size - (state.queen.row - state.queen.column) + 1 
    } else { 
        state.size + 1 
    };
    let closest_obstacle_column = state.obstacles
        .iter()
        .filter(|p| p.column > state.queen.column && p.row > state.queen.row)
        .filter(|p| state.size + p.row - p.column == diag1_down_diff)
        .map(|p| p.column)
        .min()
        .unwrap_or(default);

    closest_obstacle_column - state.queen.column - 1
}

fn diag2_up_available(state: &State) -> usize {
    let diag2_up_sum = state.queen.row + state.queen.column;
    let default = if diag2_up_sum > state.size { state.size + 1 } else { diag2_up_sum };
    let closest_obstacle_column = state.obstacles
        .iter()
        .filter(|p| p.column > state.queen.column && p.row < state.queen.row)
        .filter(|p| p.column + p.row == diag2_up_sum)
        .map(|p| p.column)
        .min()
        .unwrap_or(default);

    closest_obstacle_column - state.queen.column - 1
}

fn diag2_down_available(state: &State) -> usize {
    let diag2_down_sum = state.queen.row + state.queen.column;
    let default = if diag2_down_sum > state.size { diag2_down_sum - state.size - 1 } else { 0 };
    let closest_obstacle_column = state.obstacles
        .iter()
        .filter(|p| p.column < state.queen.column && p.row > state.queen.row)
        .filter(|p| p.column + p.row == diag2_down_sum)
        .map(|p| p.column)
        .max()
        .unwrap_or(default);

    state.queen.column - closest_obstacle_column - 1
}

fn solution(state: State) -> usize {
    row_left_available(&state) + row_right_available(&state) +
    column_up_available(&state) + column_down_available(&state) +
    diag1_up_available(&state) + diag1_down_available(&state) +
    diag2_up_available(&state) + diag2_down_available(&state)
}

fn main() {
    let n = read!();
    let k = read!();
    let queen = Point::new(read!(), read!());
    let obstacles = (0..k).map(|_| Point::new(read!(),read!())).collect();
    let result = solution(State{size: n, queen, obstacles});
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_column_up_several() {
        let state = State {
            size: 10,
            queen: Point::new(9, 4),
            obstacles: vec![Point::new(7, 4), Point::new(6, 4)],
        };
        assert_eq!(column_up_available(&state), 1);
    }

    #[test]
    fn test_column_down_several() {
        let state = State {
            size: 10,
            queen: Point::new(1, 4),
            obstacles: vec![Point::new(7, 4), Point::new(6, 4)],
        };
        assert_eq!(column_down_available(&state), 4);
    }


    #[test]
    fn test_row_left_available() {
        let state = State {
            size: 5,
            queen: Point::new(1, 4),
            obstacles: vec![],
        };
        assert_eq!(row_left_available(&state), 3);

        let state = State {
            size: 5,
            queen: Point::new(1, 4),
            obstacles: vec![Point::new(2, 3), Point::new(1, 2)],
        };
        assert_eq!(row_left_available(&state), 1);
    }

    #[test]
    fn test_row_right_available() {
        let state = State {
            size: 5,
            queen: Point::new(1, 2),
            obstacles: vec![],
        };
        assert_eq!(row_right_available(&state), 3);

        let state = State {
            size: 5,
            queen: Point::new(1, 2),
            obstacles: vec![Point::new(1, 1), Point::new(1, 4)],
        };
        assert_eq!(row_right_available(&state), 1);
    }

    #[test]
    fn test_sample_input_0() {
        let state = State {
            size: 4,
            queen: Point::new(4, 4),
            obstacles: vec![],
        };
        assert_eq!(solution(state), 9);
    }

    #[test]
    fn test_sample_input_1() {
        let state = State {
            size: 5,
            queen: Point::new(4, 3),
            obstacles: vec![
                Point::new(5, 5),
                Point::new(4, 2),
                Point::new(2, 3),
            ],
        };
        assert_eq!(row_left_available(&state), 0);
        assert_eq!(row_right_available(&state), 2);
        assert_eq!(column_up_available(&state), 1);
        assert_eq!(column_down_available(&state), 1);
        assert_eq!(diag1_up_available(&state), 2);
        assert_eq!(diag1_down_available(&state), 1);
        assert_eq!(diag2_up_available(&state), 2);
        assert_eq!(diag2_down_available(&state), 1);
        assert_eq!(solution(state), 10);
    }

    #[test]
    fn test_sample_input_2() {
        let state = State {
            size: 1,
            queen: Point::new(1, 1),
            obstacles: vec![],
        };
        assert_eq!(solution(state), 0);
    }

    #[test]
    fn test_zero() {
        let state = State {
            size: 10,
            queen: Point::new(2, 2),
            obstacles: vec![
                Point::new(2, 1),
                Point::new(2, 3),
                Point::new(1, 2),
                Point::new(3, 2),
                Point::new(1, 1),
                Point::new(3, 3),
                Point::new(3, 1),
                Point::new(1, 3),
            ],
        };

        assert_eq!(solution(state), 0);
        
        let state = State {
            size: 10,
            queen: Point::new(2, 8),
            obstacles: vec![
                Point::new(2, 7),
                Point::new(2, 9),
                Point::new(1, 8),
                Point::new(3, 8),
                Point::new(1, 7),
                Point::new(3, 7),
                Point::new(1, 9),
                Point::new(3, 9),
            ],
        };

        assert_eq!(solution(state), 0);

        let state = State {
            size: 10,
            queen: Point::new(8, 8),
            obstacles: vec![
                Point::new(8, 7),
                Point::new(8, 9),
                Point::new(7, 8),
                Point::new(9, 8),
                Point::new(7, 7),
                Point::new(9, 7),
                Point::new(7, 9),
                Point::new(9, 9),
            ],
        };

        assert_eq!(solution(state), 0);

        let state = State {
            size: 10,
            queen: Point::new(8, 3),
            obstacles: vec![
                Point::new(8, 2),
                Point::new(8, 4),
                Point::new(7, 3),
                Point::new(9, 3),
                Point::new(7, 2),
                Point::new(9, 2),
                Point::new(7, 4),
                Point::new(9, 4),
            ],
        };

        assert_eq!(solution(state), 0);
    }

    #[test]
    fn test_full() {
        let points = vec![Point::new(3, 3), Point::new(3, 8), Point::new(8, 8), Point::new(8, 3)];
        for point in points {
            let state = State {
                size: 10,
                queen: point,
                obstacles: vec![],
            };
            assert_eq!(solution(state), 31);
        }
    }

    #[test]
    fn test_diag2_down_defaults() {
        let mut state = State {
            size: 10,
            queen: Point::new(4, 3),
            obstacles: vec![],
        };
        assert_eq!(diag2_down_available(&state), 2);

        state.queen = Point::new(4, 8);
        assert_eq!(diag2_down_available(&state), 6);
    
        state.queen = Point::new(7, 8);
        assert_eq!(diag2_down_available(&state), 3);

        state.queen = Point::new(7, 3);
        assert_eq!(diag2_down_available(&state), 2);
    }
    
    #[test]
    fn test_diag2_down_several() {
        let mut state = State {
            size: 10,
            queen: Point::new(4, 3),
            obstacles: vec![Point::new(5, 2), Point::new(6, 1)],
        };
        assert_eq!(diag2_down_available(&state), 0);

        state.queen = Point::new(4, 8);
        state.obstacles = vec![Point::new(6, 6), Point::new(8, 4)];
        assert_eq!(diag2_down_available(&state), 1);
    
        state.queen = Point::new(7, 8);
        state.obstacles = vec![Point::new(9, 6), Point::new(10, 5)];
        assert_eq!(diag2_down_available(&state), 1);

        state.queen = Point::new(7, 3);
        state.obstacles = vec![Point::new(8, 2), Point::new(9, 1)];
        assert_eq!(diag2_down_available(&state), 0);
    }

    #[test]
    fn test_diag1_down_defaults() {
        let mut state = State {
            size: 10,
            queen: Point::new(4, 3),
            obstacles: vec![],
        };
        assert_eq!(diag1_down_available(&state), 6);

        state.queen = Point::new(4, 8);
        assert_eq!(diag1_down_available(&state), 2);
    
        state.queen = Point::new(7, 8);
        assert_eq!(diag1_down_available(&state), 2);

        state.queen = Point::new(7, 3);
        assert_eq!(diag1_down_available(&state), 3);
    }
    
    #[test]
    fn test_diag1_down_several() {
        let mut state = State {
            size: 10,
            queen: Point::new(4, 3),
            obstacles: vec![Point::new(6, 5), Point::new(8, 7)],
        };
        assert_eq!(diag1_down_available(&state), 1);

        state.queen = Point::new(4, 8);
        state.obstacles = vec![Point::new(5, 9), Point::new(6, 10)];
        assert_eq!(diag1_down_available(&state), 0);
    
        state.queen = Point::new(7, 8);
        state.obstacles = vec![Point::new(8, 9), Point::new(9, 10)];
        assert_eq!(diag1_down_available(&state), 0);

        state.queen = Point::new(7, 3);
        state.obstacles = vec![Point::new(9, 5), Point::new(10, 6)];
        assert_eq!(diag1_down_available(&state), 1);
    }

    #[test]
    fn test_diag2_up_defaults() {
        let mut state = State {
            size: 10,
            queen: Point::new(4, 3),
            obstacles: vec![],
        };
        assert_eq!(diag2_up_available(&state), 3);

        state.queen = Point::new(4, 8);
        assert_eq!(diag2_up_available(&state), 2);
    
        state.queen = Point::new(7, 8);
        assert_eq!(diag2_up_available(&state), 2);

        state.queen = Point::new(7, 3);
        assert_eq!(diag2_up_available(&state), 6);
    }
    
    #[test]
    fn test_diag2_up_several() {
        let mut state = State {
            size: 10,
            queen: Point::new(4, 3),
            obstacles: vec![Point::new(1, 6), Point::new(2, 5)],
        };
        assert_eq!(diag2_up_available(&state), 1);

        state.queen = Point::new(4, 8);
        state.obstacles = vec![Point::new(2, 10), Point::new(3, 9)];
        assert_eq!(diag2_up_available(&state), 0);
    
        state.queen = Point::new(7, 8);
        state.obstacles = vec![Point::new(6, 9), Point::new(5, 10)];
        assert_eq!(diag2_up_available(&state), 0);

        state.queen = Point::new(7, 3);
        state.obstacles = vec![Point::new(3, 7), Point::new(5, 5)];
        assert_eq!(diag2_up_available(&state), 1);
    }

    #[test]
    fn test_diag1_up_defaults() {
        let mut state = State {
            size: 10,
            queen: Point::new(4, 3),
            obstacles: vec![],
        };
        assert_eq!(diag1_up_available(&state), 2);

        state.queen = Point::new(4, 8);
        assert_eq!(diag1_up_available(&state), 3);
    
        state.queen = Point::new(7, 8);
        assert_eq!(diag1_up_available(&state), 6);

        state.queen = Point::new(7, 3);
        assert_eq!(diag1_up_available(&state), 2);
    }
    
    #[test]
    fn test_diag1_up_several() {
        let mut state = State {
            size: 10,
            queen: Point::new(4, 3),
            obstacles: vec![Point::new(3, 2), Point::new(2, 1)],
        };
        assert_eq!(diag1_up_available(&state), 0);

        state.queen = Point::new(4, 8);
        state.obstacles = vec![Point::new(2, 6), Point::new(1, 5)];
        assert_eq!(diag1_up_available(&state), 1);
    
        state.queen = Point::new(7, 8);
        state.obstacles = vec![Point::new(3, 4), Point::new(5, 6)];
        assert_eq!(diag1_up_available(&state), 1);

        state.queen = Point::new(7, 3);
        state.obstacles = vec![Point::new(6, 2), Point::new(5, 1)];
        assert_eq!(diag1_up_available(&state), 0);
    }

    #[test]
    fn test_diag1_up() {
        let state = State {
            size: 10,
            queen: Point::new(4, 5),
            obstacles: vec![Point::new(3, 2)],
        };

        assert_eq!(diag1_up_available(&state), 3);
    }
}