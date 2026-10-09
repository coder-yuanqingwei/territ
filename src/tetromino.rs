use macroquad::color::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

const SHAPES: [[[(i32, i32); 4]; 4]; 7] = [
    [
        [(0, 1), (1, 1), (2, 1), (3, 1)],
        [(2, 0), (2, 1), (2, 2), (2, 3)],
        [(0, 2), (1, 2), (2, 2), (3, 2)],
        [(1, 0), (1, 1), (1, 2), (1, 3)],
    ],
    [
        [(1, 0), (2, 0), (1, 1), (2, 1)],
        [(1, 0), (2, 0), (1, 1), (2, 1)],
        [(1, 0), (2, 0), (1, 1), (2, 1)],
        [(1, 0), (2, 0), (1, 1), (2, 1)],
    ],
    [
        [(1, 0), (0, 1), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (2, 1), (1, 2)],
        [(0, 1), (1, 1), (2, 1), (1, 2)],
        [(1, 0), (0, 1), (1, 1), (1, 2)],
    ],
    [
        [(1, 0), (2, 0), (0, 1), (1, 1)],
        [(0, 0), (0, 1), (1, 1), (1, 2)],
        [(1, 1), (2, 1), (0, 2), (1, 2)],
        [(1, 0), (1, 1), (2, 1), (2, 2)],
    ],
    [
        [(0, 0), (1, 0), (1, 1), (2, 1)],
        [(2, 0), (1, 1), (2, 1), (1, 2)],
        [(0, 1), (1, 1), (1, 2), (2, 2)],
        [(1, 0), (0, 1), (1, 1), (0, 2)],
    ],
    [
        [(0, 0), (0, 1), (1, 1), (2, 1)],
        [(1, 0), (2, 0), (1, 1), (1, 2)],
        [(0, 1), (1, 1), (2, 1), (2, 2)],
        [(1, 0), (1, 1), (0, 2), (1, 2)],
    ],
    [
        [(2, 0), (0, 1), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (1, 2), (2, 2)],
        [(0, 1), (1, 1), (2, 1), (0, 2)],
        [(0, 0), (1, 0), (1, 1), (1, 2)],
    ],
];

const TYPE_INDEX: [TetrominoType; 7] = [
    TetrominoType::I,
    TetrominoType::O,
    TetrominoType::T,
    TetrominoType::S,
    TetrominoType::Z,
    TetrominoType::J,
    TetrominoType::L,
];

fn type_to_index(t: TetrominoType) -> usize {
    match t {
        TetrominoType::I => 0,
        TetrominoType::O => 1,
        TetrominoType::T => 2,
        TetrominoType::S => 3,
        TetrominoType::Z => 4,
        TetrominoType::J => 5,
        TetrominoType::L => 6,
    }
}

pub fn get_shape(t: TetrominoType, rotation: usize) -> [(i32, i32); 4] {
    SHAPES[type_to_index(t)][rotation % 4]
}

pub fn get_color(t: TetrominoType) -> Color {
    match t {
        TetrominoType::I => Color::from_rgba(0, 240, 240, 255),
        TetrominoType::O => Color::from_rgba(240, 240, 0, 255),
        TetrominoType::T => Color::from_rgba(180, 0, 255, 255),
        TetrominoType::S => Color::from_rgba(0, 240, 0, 255),
        TetrominoType::Z => Color::from_rgba(240, 0, 0, 255),
        TetrominoType::J => Color::from_rgba(0, 0, 240, 255),
        TetrominoType::L => Color::from_rgba(240, 160, 0, 255),
    }
}

pub fn random_type() -> TetrominoType {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    TYPE_INDEX[rng.gen_range(0..7)]
}

