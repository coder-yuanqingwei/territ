use crate::tetromino::{get_shape, TetrominoType};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

pub struct Board {
    grid: [[Option<TetrominoType>; WIDTH]; HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Board {
            grid: [[None; WIDTH]; HEIGHT],
        }
    }

    pub fn cell(&self, row: usize, col: usize) -> Option<TetrominoType> {
        self.grid[row][col]
    }

    pub fn is_valid_position(&self, t: TetrominoType, rotation: usize, pos: (i32, i32)) -> bool {
        let shape = get_shape(t, rotation);
        for &(dx, dy) in &shape {
            let x = pos.0 + dx;
            let y = pos.1 + dy;
            if x < 0 || x >= WIDTH as i32 || y < 0 || y >= HEIGHT as i32 {
                return false;
            }
            if self.grid[y as usize][x as usize].is_some() {
                return false;
            }
        }
        true
    }

    pub fn lock_piece(&mut self, t: TetrominoType, rotation: usize, pos: (i32, i32)) {
        let shape = get_shape(t, rotation);
        for &(dx, dy) in &shape {
            let x = pos.0 + dx;
            let y = pos.1 + dy;
            if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
                self.grid[y as usize][x as usize] = Some(t);
            }
        }
    }

    pub fn full_rows(&self) -> Vec<usize> {
        (0..HEIGHT)
            .filter(|&y| (0..WIDTH).all(|x| self.grid[y][x].is_some()))
            .collect()
    }

    pub fn clear_rows(&mut self, rows: &[usize]) -> u32 {
        if rows.is_empty() {
            return 0;
        }
        let remove: std::collections::HashSet<usize> = rows.iter().copied().collect();
        let mut new_grid = [[None; WIDTH]; HEIGHT];
        let mut write = HEIGHT;
        for y in (0..HEIGHT).rev() {
            if !remove.contains(&y) {
                write -= 1;
                new_grid[write] = self.grid[y];
            }
        }
        let cleared = write as u32;
        self.grid = new_grid;
        cleared
    }

    pub fn is_game_over(&self, t: TetrominoType, rotation: usize, pos: (i32, i32)) -> bool {
        !self.is_valid_position(t, rotation, pos)
    }

    pub fn reset(&mut self) {
        self.grid = [[None; WIDTH]; HEIGHT];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fill_row(board: &mut Board, row: usize, t: TetrominoType) {
        for col in 0..WIDTH {
            board.grid[row][col] = Some(t);
        }
    }

    #[test]
    fn full_rows_detects_complete_rows() {
        let mut board = Board::new();
        board.grid[18][0] = Some(TetrominoType::I);
        assert_eq!(board.full_rows(), Vec::<usize>::new());
        fill_row(&mut board, 19, TetrominoType::I);
        assert_eq!(board.full_rows(), vec![19]);
    }

    #[test]
    fn clear_rows_removes_and_shifts() {
        let mut board = Board::new();
        fill_row(&mut board, 19, TetrominoType::I);
        board.grid[17][0] = Some(TetrominoType::T);

        assert_eq!(board.clear_rows(&[19]), 1);
        assert!(board.full_rows().is_empty());
        assert_eq!(board.cell(18, 0), Some(TetrominoType::T));
        assert_eq!(board.cell(19, 0), None);
    }

    #[test]
    fn clear_rows_handles_multiple_non_adjacent() {
        let mut board = Board::new();
        fill_row(&mut board, 15, TetrominoType::I);
        board.grid[18][0] = Some(TetrominoType::O);
        fill_row(&mut board, 19, TetrominoType::J);
        board.grid[18][5] = Some(TetrominoType::T);

        assert_eq!(board.clear_rows(&[15, 19]), 2);
        assert_eq!(board.cell(19, 0), Some(TetrominoType::O));
        assert_eq!(board.cell(19, 5), Some(TetrominoType::T));
        assert_eq!(board.cell(18, 0), None);
    }

    #[test]
    fn clear_rows_empty_slice_noop() {
        let mut board = Board::new();
        board.grid[0][1] = Some(TetrominoType::T);
        assert_eq!(board.clear_rows(&[]), 0);
        assert_eq!(board.cell(0, 1), Some(TetrominoType::T));
    }
}
