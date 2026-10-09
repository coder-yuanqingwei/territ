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

    pub fn clear_lines(&mut self) -> u32 {
        let mut cleared = 0u32;
        let mut y = HEIGHT as i32 - 1;
        while y >= 0 {
            let row_full = (0..WIDTH).all(|x| self.grid[y as usize][x].is_some());
            if row_full {
                cleared += 1;
                for row in (1..=y as usize).rev() {
                    self.grid[row] = self.grid[row - 1];
                }
                self.grid[0] = [None; WIDTH];
            } else {
                y -= 1;
            }
        }
        cleared
    }

    pub fn is_game_over(&self, t: TetrominoType, rotation: usize, pos: (i32, i32)) -> bool {
        !self.is_valid_position(t, rotation, pos)
    }

    pub fn reset(&mut self) {
        self.grid = [[None; WIDTH]; HEIGHT];
    }
}
