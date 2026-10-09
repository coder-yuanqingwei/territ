use crate::board::Board;
use crate::tetromino::{get_shape, Bag, TetrominoType};
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};

pub const CLEAR_DURATION: f64 = 0.35;

#[derive(Debug)]
pub enum GameEvent {
    Rotated,
    HardDropped,
    Held,
    LinesCleared { rows: Vec<usize>, points: u32 },
    LevelUp { level: u32 },
    GameOver,
}

pub struct Game {
    pub board: Board,
    pub current_type: TetrominoType,
    pub current_rotation: usize,
    pub pos: (i32, i32),
    pub next_type: TetrominoType,
    pub hold_type: Option<TetrominoType>,
    pub hold_used: bool,
    pub clear_timer: f64,
    pub pending_clear_rows: Vec<usize>,
    pub score: u32,
    pub level: u32,
    pub lines_cleared: u32,
    pub game_over: bool,
    pub paused: bool,
    bag: Bag,
    events: Vec<GameEvent>,
    fall_timer: f64,
    fall_interval: f64,
    das_timer_left: f64,
    das_timer_right: f64,
    das_timer_down: f64,
}

impl Game {
    pub fn new() -> Self {
        let mut bag = Bag::new();
        let current_type = bag.next();
        let next_type = bag.next();
        let pos = Self::spawn_pos(current_type);
        Game {
            board: Board::new(),
            current_type,
            current_rotation: 0,
            pos,
            next_type,
            hold_type: None,
            hold_used: false,
            clear_timer: 0.0,
            pending_clear_rows: Vec::new(),
            score: 0,
            level: 1,
            lines_cleared: 0,
            game_over: false,
            paused: false,
            bag,
            events: Vec::new(),
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
        self.next_type = self.bag.next();
        self.current_rotation = 0;
        self.pos = Self::spawn_pos(self.current_type);
        if self.board.is_game_over(self.current_type, self.current_rotation, self.pos) {
            self.game_over = true;
            self.events.push(GameEvent::GameOver);
        }
    }

    pub fn hold_swap(&mut self) {
        if self.game_over || self.paused || self.hold_used || self.is_clearing() {
            return;
        }
        let incoming = match self.hold_type.take() {
            Some(t) => t,
            None => {
                let t = self.next_type;
                self.next_type = self.bag.next();
                t
            }
        };
        self.hold_type = Some(self.current_type);
        self.current_type = incoming;
        self.current_rotation = 0;
        self.pos = Self::spawn_pos(incoming);
        self.hold_used = true;
        self.fall_timer = 0.0;
        self.events.push(GameEvent::Held);
        if self.board.is_game_over(self.current_type, self.current_rotation, self.pos) {
            self.game_over = true;
            self.events.push(GameEvent::GameOver);
        }
    }

    pub fn ghost_position(&self) -> (i32, i32) {
        let mut pos = self.pos;
        while self
            .board
            .is_valid_position(self.current_type, self.current_rotation, (pos.0, pos.1 + 1))
        {
            pos.1 += 1;
        }
        pos
    }

    pub fn is_clearing(&self) -> bool {
        self.clear_timer > 0.0
    }

    pub fn take_events(&mut self) -> Vec<GameEvent> {
        std::mem::take(&mut self.events)
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
        self.board
            .lock_piece(self.current_type, self.current_rotation, self.pos);
        self.hold_used = false;
        let full = self.board.full_rows();
        if full.is_empty() {
            self.spawn_piece();
        } else {
            self.pending_clear_rows = full;
            self.clear_timer = CLEAR_DURATION;
        }
    }

    fn finish_clearing(&mut self) {
        self.clear_timer = 0.0;
        let rows = std::mem::take(&mut self.pending_clear_rows);
        let cleared = self.board.clear_rows(&rows);
        self.lines_cleared += cleared;
        let points = match cleared {
            1 => 100,
            2 => 300,
            3 => 500,
            _ => 800,
        } * self.level;
        self.score += points;
        self.events.push(GameEvent::LinesCleared { rows, points });
        let new_level = 1 + self.lines_cleared / 10;
        if new_level > self.level {
            self.level = new_level;
            self.fall_interval = (0.8 - (self.level as f64 - 1.0) * 0.07).max(0.1);
            self.events.push(GameEvent::LevelUp { level: self.level });
        }
        self.spawn_piece();
    }

    pub fn tick(&mut self, dt: f64) {
        if self.game_over || self.paused {
            return;
        }

        if self.clear_timer > 0.0 {
            self.clear_timer -= dt;
            if self.clear_timer <= 0.0 {
                self.finish_clearing();
            }
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

        if self.is_clearing() {
            return;
        }

        if is_key_pressed(KeyCode::C) {
            self.hold_swap();
            return;
        }

        if is_key_pressed(KeyCode::Up) && self.rotate() {
            self.events.push(GameEvent::Rotated);
        }

        if is_key_pressed(KeyCode::Space) {
            self.events.push(GameEvent::HardDropped);
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
        self.hold_type = None;
        self.hold_used = false;
        self.clear_timer = 0.0;
        self.pending_clear_rows.clear();
        self.events.clear();
        self.bag = Bag::new();
        self.current_type = self.bag.next();
        self.next_type = self.bag.next();
        self.current_rotation = 0;
        self.pos = Self::spawn_pos(self.current_type);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hold_swap_moves_current_to_hold_and_spawns_next() {
        let mut game = Game::new();
        let current = game.current_type;
        let next = game.next_type;
        assert!(game.hold_type.is_none());

        game.hold_swap();
        assert_eq!(game.hold_type, Some(current));
        assert_eq!(game.current_type, next);
        assert!(game.hold_used);

        game.hold_swap();
        assert_eq!(game.current_type, next);
        assert_eq!(game.hold_type, Some(current));
    }

    #[test]
    fn hold_swap_back_exchanges_pieces() {
        let mut game = Game::new();
        let first = game.current_type;
        game.hold_swap();
        let second = game.current_type;
        game.hold_used = false;
        game.hold_swap();
        assert_eq!(game.current_type, first);
        assert_eq!(game.hold_type, Some(second));
    }

    #[test]
    fn ghost_position_is_final_landing_spot() {
        let game = Game::new();
        let ghost = game.ghost_position();
        assert!(
            game.board
                .is_valid_position(game.current_type, game.current_rotation, ghost)
        );
        assert!(
            !game.board
                .is_valid_position(game.current_type, game.current_rotation, (ghost.0, ghost.1 + 1))
        );
        assert!(ghost.1 >= game.pos.1);
    }

    #[test]
    fn clearing_flow_scores_and_spawns_after_delay() {
        let mut game = Game::new();
        for col in 0..10 {
            game.board
                .lock_piece(TetrominoType::I, 0, (col, 17));
        }
        assert_eq!(game.board.full_rows(), vec![18]);

        game.hard_drop();
        assert!(game.is_clearing());
        assert_eq!(game.score, 0);

        game.tick(0.36);
        assert!(!game.is_clearing());
        assert_eq!(game.lines_cleared, 1);
        assert_eq!(game.score, 100);

        let events = game.take_events();
        assert!(matches!(
            events.as_slice(),
            [GameEvent::LinesCleared { .. }]
        ));
    }
}
