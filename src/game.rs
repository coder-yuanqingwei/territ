use crate::board::Board;
use crate::tetromino::{get_shape, random_type, TetrominoType};
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};

pub struct Game {
    pub board: Board,
    pub current_type: TetrominoType,
    pub current_rotation: usize,
    pub pos: (i32, i32),
    pub next_type: TetrominoType,
    pub score: u32,
    pub level: u32,
    pub lines_cleared: u32,
    pub game_over: bool,
    pub paused: bool,
    fall_timer: f64,
    fall_interval: f64,
    das_timer_left: f64,
    das_timer_right: f64,
    das_timer_down: f64,
}

impl Game {
    pub fn new() -> Self {
        let current_type = random_type();
        let next_type = random_type();
        let pos = Self::spawn_pos(current_type);
        Game {
            board: Board::new(),
            current_type,
            current_rotation: 0,
            pos,
            next_type,
            score: 0,
            level: 1,
            lines_cleared: 0,
            game_over: false,
            paused: false,
            fall_timer: 0.0,
            fall_interval: 0.8,
            das_timer_left: 0.0,
            das_timer_right: 0.0,
            das_timer_down: 0.0,
        }
    }

    fn spawn_pos(t: TetrominoType) -> (i32, i32) {
        let shape = get_shape(t, 0);
        let min_x = shape.iter().map(|&(x, _)| x).min().unwrap_or(0);
        let max_x = shape.iter().map(|&(x, _)| x).max().unwrap_or(0);
        let center = (10 - (max_x - min_x + 1)) / 2;
        (center, 0)
    }

    pub fn spawn_piece(&mut self) {
        self.current_type = self.next_type;
        self.next_type = random_type();
        self.current_rotation = 0;
        self.pos = Self::spawn_pos(self.current_type);
        if self.board.is_game_over(self.current_type, self.current_rotation, self.pos) {
            self.game_over = true;
        }
    }

    pub fn move_left(&mut self) -> bool {
        let new_pos = (self.pos.0 - 1, self.pos.1);
        if self.board.is_valid_position(self.current_type, self.current_rotation, new_pos) {
            self.pos = new_pos;
            true
        } else {
            false
        }
    }

    pub fn move_right(&mut self) -> bool {
        let new_pos = (self.pos.0 + 1, self.pos.1);
        if self.board.is_valid_position(self.current_type, self.current_rotation, new_pos) {
            self.pos = new_pos;
            true
        } else {
            false
        }
    }

    pub fn move_down(&mut self) -> bool {
        let new_pos = (self.pos.0, self.pos.1 + 1);
        if self.board.is_valid_position(self.current_type, self.current_rotation, new_pos) {
            self.pos = new_pos;
            true
        } else {
            false
        }
    }

    pub fn rotate(&mut self) -> bool {
        let new_rotation = (self.current_rotation + 1) % 4;
        if self.board.is_valid_position(self.current_type, new_rotation, self.pos) {
            self.current_rotation = new_rotation;
            true
        } else {
            false
        }
    }

    pub fn hard_drop(&mut self) {
        while self.move_down() {}
        self.lock_and_next();
    }

    fn lock_and_next(&mut self) {
        self.board.lock_piece(self.current_type, self.current_rotation, self.pos);
        let cleared = self.board.clear_lines();
        if cleared > 0 {
            self.lines_cleared += cleared;
            let points = match cleared {
                1 => 100,
                2 => 300,
                3 => 500,
                _ => 800,
            };
            self.score += points * self.level;
            self.level = 1 + self.lines_cleared / 10;
            self.fall_interval = (0.8 - (self.level as f64 - 1.0) * 0.07).max(0.1);
        }
        self.spawn_piece();
    }

    pub fn tick(&mut self, dt: f64) {
        if self.game_over || self.paused {
            return;
        }

        self.fall_timer += dt;
        if self.fall_timer >= self.fall_interval {
            self.fall_timer = 0.0;
            if !self.move_down() {
                self.lock_and_next();
            }
        }
    }

    pub fn handle_input(&mut self, dt: f64) {
        if is_key_pressed(KeyCode::P) {
            self.paused = !self.paused;
        }

        if self.game_over {
            if is_key_pressed(KeyCode::R) {
                self.restart();
            }
            return;
        }

        if self.paused {
            return;
        }

        if is_key_pressed(KeyCode::Up) {
            self.rotate();
        }

        if is_key_pressed(KeyCode::Space) {
            self.hard_drop();
        }

        const DAS_DELAY: f64 = 0.17;
        const DAS_REPEAT: f64 = 0.05;

        if is_key_down(KeyCode::Left) {
            self.das_timer_left += dt;
            if is_key_pressed(KeyCode::Left) {
                self.move_left();
                self.das_timer_left = 0.0;
            } else if self.das_timer_left >= DAS_DELAY {
                let elapsed = self.das_timer_left - DAS_DELAY;
                if elapsed % DAS_REPEAT < dt {
                    self.move_left();
                }
            }
        } else {
            self.das_timer_left = 0.0;
        }

        if is_key_down(KeyCode::Right) {
            self.das_timer_right += dt;
            if is_key_pressed(KeyCode::Right) {
                self.move_right();
                self.das_timer_right = 0.0;
            } else if self.das_timer_right >= DAS_DELAY {
                let elapsed = self.das_timer_right - DAS_DELAY;
                if elapsed % DAS_REPEAT < dt {
                    self.move_right();
                }
            }
        } else {
            self.das_timer_right = 0.0;
        }

        if is_key_down(KeyCode::Down) {
            self.das_timer_down += dt;
            if is_key_pressed(KeyCode::Down) {
                if self.move_down() {
                    self.score += 1;
                    self.fall_timer = 0.0;
                }
                self.das_timer_down = 0.0;
            } else if self.das_timer_down >= DAS_DELAY {
                let elapsed = self.das_timer_down - DAS_DELAY;
                if elapsed % DAS_REPEAT < dt && self.move_down() {
                    self.score += 1;
                    self.fall_timer = 0.0;
                }
            }
        } else {
            self.das_timer_down = 0.0;
        }
    }

    pub fn restart(&mut self) {
        self.board.reset();
        self.score = 0;
        self.level = 1;
        self.lines_cleared = 0;
        self.game_over = false;
        self.paused = false;
        self.fall_timer = 0.0;
        self.fall_interval = 0.8;
        self.spawn_piece();
    }
}
