use std::cmp::min;
use text_io::*;

struct Matrix {
    m: Vec<Vec<usize>>,
}

impl Matrix {
    fn from_vec(v: Vec<Vec<usize>>) -> Matrix {
        Matrix{m: v}
    }

    fn to_vec(self) -> Vec<Vec<usize>> {
        self.m
    }

    fn widht(&self) -> usize {
        self.m[0].len()
    }

    fn height(&self) -> usize {
        self.m.len()
    }

    fn num_of_layers(&self) -> usize {
        min(self.height(), self.widht()) / 2
    }

    fn layer_top_start_coord(&self, layer: usize) -> (usize, usize) {
        assert!(layer < self.num_of_layers());
        (layer, layer)
    }

    fn layer_right_start_coord(&self, layer: usize) -> (usize, usize) {
        assert!(layer < self.num_of_layers());
        (layer, self.widht() - 1 - layer)
    }

    fn layer_bottom_start_coord(&self, layer: usize) -> (usize, usize) {
        assert!(layer < self.num_of_layers());
        (self.height() - 1 - layer, self.widht() - 1 - layer)
    }
    
    fn layer_left_start_coord(&self, layer: usize) -> (usize, usize) {
        assert!(layer < self.num_of_layers());
        (self.height() - 1 - layer, layer)
    }
    
    fn read_layer(&self, layer: usize) -> Vec<usize> {
        let mut result = Vec::new();
        let (top_layer_i, top_layer_j) = self.layer_top_start_coord(layer);
        let (right_layer_i, right_layer_j) = self.layer_right_start_coord(layer);
        let (bottom_layer_i, bottom_layer_j) = self.layer_bottom_start_coord(layer);
        let (left_layer_i, left_layer_j) = self.layer_left_start_coord(layer);

        for j in top_layer_j..right_layer_j {
            result.push(self.m[top_layer_i][j]);
        }

        for i in right_layer_i..bottom_layer_i {
            result.push(self.m[i][right_layer_j])
        }

        for nth_item in 0..bottom_layer_j - left_layer_j {
            let j_idx = bottom_layer_j - nth_item;
            result.push(self.m[bottom_layer_i][j_idx]);
        }

        for nth_item in 0..left_layer_i - top_layer_i {
            let i_idx = bottom_layer_i - nth_item;
            result.push(self.m[i_idx][left_layer_j]);
        }

        result
    }

    fn write_layer(&mut self, layer: usize, layer_data: Vec<usize>) {
        let (top_layer_i, top_layer_j) = self.layer_top_start_coord(layer);
        let (right_layer_i, right_layer_j) = self.layer_right_start_coord(layer);
        let (bottom_layer_i, bottom_layer_j) = self.layer_bottom_start_coord(layer);
        let (left_layer_i, left_layer_j) = self.layer_left_start_coord(layer);

        let mut data_idx = 0;
        for j in top_layer_j..right_layer_j {
            self.m[top_layer_i][j] = layer_data[data_idx];
            data_idx += 1;
        }

        for i in right_layer_i..bottom_layer_i {
            self.m[i][right_layer_j] = layer_data[data_idx];
            data_idx += 1;
        }

        for nth_item in 0..bottom_layer_j - left_layer_j {
            let j_idx = bottom_layer_j - nth_item;
            self.m[bottom_layer_i][j_idx] = layer_data[data_idx];
            data_idx += 1;
        }

        for nth_item in 0..left_layer_i - top_layer_i {
            let i_idx = bottom_layer_i - nth_item;
            self.m[i_idx][left_layer_j] = layer_data[data_idx];
            data_idx += 1;
        }
    }

    fn rotate(&mut self, r: usize) {
        for i in 0..self.num_of_layers() {
            let mut layer = self.read_layer(i);
            let rotations = r % layer.len();
            layer.rotate_left(rotations);
            self.write_layer(i, layer);
        }
    }
}


fn solution(v: Vec<Vec<usize>>, r: usize) -> Vec<Vec<usize>> {
    let mut matrix = Matrix::from_vec(v);
    matrix.rotate(r);
    matrix.to_vec()
}

fn main() {
    let rows = read!();
    let columns = read!();
    let r = read!();
    let input = (0..rows)
        .map(|_| (0..columns).map(|_| read!()).collect())
        .collect();

    let result = solution(input, r);

    for row in result {
        for item in row {
            print!("{} ", item);
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_layer() {
        let input = Matrix::from_vec(vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
        ]);

        assert_eq!(input.read_layer(1), vec![2, 3, 3, 2]);
        assert_eq!(input.read_layer(0), vec![1, 2, 3, 4, 4, 4, 4, 3, 2, 1, 1, 1]);
        
        let input = Matrix::from_vec(vec![
            vec![2, 3],
            vec![2, 4],
            vec![1, 3],
            vec![1, 2],
        ]);
        assert_eq!(input.read_layer(0), vec![2, 3, 4, 3, 2, 1, 1, 2]);
    }

    #[test]
    fn test_sample_input_0() {
        let input = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];

        let result = solution(input, 2);

        let expected = vec![
            vec![3, 4, 8, 12],
            vec![2, 11, 10, 16],
            vec![1, 7, 6, 15],
            vec![5, 9, 13, 14],
        ];

        assert_eq!(result, expected);
    }
}