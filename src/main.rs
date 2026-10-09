mod board;
mod game;
mod scores;
mod tetromino;

use game::Game;
use macroquad::prelude::*;
use scores::{scores_path, HighScores};
use tetromino::{get_color, get_shape};

const CELL_SIZE: f32 = 30.0;
const BOARD_OFFSET_X: f32 = 30.0;
const BOARD_OFFSET_Y: f32 = 30.0;
const PANEL_X: f32 = 350.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        window_width: 500,
        window_height: 700,
        window_resizable: false,
        ..Default::default()
    }
}

fn draw_ui_text(font: &Font, text: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font: Some(font),
            font_size: size as u16,
            font_scale: 1.0,
            color,
            ..Default::default()
        },
    );
}

fn measure_ui_text(font: &Font, text: &str, size: f32) -> TextDimensions {
    measure_text(text, Some(font), size as u16, 1.0)
}

enum Phase {
    Playing,
    EnteringName(String),
    GameOver,
}

const MAX_NAME_LEN: usize = 8;

#[macroquad::main(window_conf = "window_conf")]
async fn main() {
    let font = load_ttf_font_from_bytes(include_bytes!("../assets/menlo_regular.ttf")).unwrap();
    let score_path = scores_path();
    let mut highscores = HighScores::load(&score_path);
    let mut game = Game::new();
    let mut phase = Phase::Playing;

    loop {
        let dt = get_frame_time() as f64;

        match &mut phase {
            Phase::Playing => {
                game.handle_input(dt);
                game.tick(dt);
                if game.game_over {
                    if highscores.qualifies(game.score) {
                        phase = Phase::EnteringName(String::new());
                    } else {
                        phase = Phase::GameOver;
                    }
                }
            }
            Phase::EnteringName(name) => {
                while let Some(c) = get_char_pressed() {
                    if c.is_ascii_alphanumeric() && name.len() < MAX_NAME_LEN {
                        name.push(c.to_ascii_uppercase());
                    }
                }
                if is_key_pressed(KeyCode::Backspace) {
                    name.pop();
                }
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
                    let name = std::mem::take(name);
                    highscores.insert(name, game.score);
                    highscores.save(&score_path);
                    phase = Phase::GameOver;
                }
            }
            Phase::GameOver => {
                game.handle_input(dt);
                if !game.game_over {
                    phase = Phase::Playing;
                }
            }
        }

        clear_background(Color::from_rgba(20, 20, 30, 255));

        draw_board_background();
        draw_locked_cells(&game);
        draw_current_piece(&game);
        draw_grid_lines();
        draw_panel(&game, &highscores, &font);

        match &phase {
            Phase::Playing => {
                if game.paused {
                    draw_overlay("PAUSED", "Press P to Resume", &font);
                }
            }
            Phase::EnteringName(name) => {
                draw_name_entry_overlay(name, game.score, &font);
            }
            Phase::GameOver => {
                draw_overlay("GAME OVER", "Press R to Restart", &font);
            }
        }

        next_frame().await
    }
}

fn draw_board_background() {
    draw_rectangle(
        BOARD_OFFSET_X - 2.0,
        BOARD_OFFSET_Y - 2.0,
        board::WIDTH as f32 * CELL_SIZE + 4.0,
        board::HEIGHT as f32 * CELL_SIZE + 4.0,
        Color::from_rgba(60, 60, 80, 255),
    );
    draw_rectangle(
        BOARD_OFFSET_X,
        BOARD_OFFSET_Y,
        board::WIDTH as f32 * CELL_SIZE,
        board::HEIGHT as f32 * CELL_SIZE,
        Color::from_rgba(10, 10, 20, 255),
    );
}

fn draw_locked_cells(game: &Game) {
    for row in 0..board::HEIGHT {
        for col in 0..board::WIDTH {
            if let Some(t) = game.board.cell(row, col) {
                draw_cell(col as f32, row as f32, get_color(t));
            }
        }
    }
}

fn draw_current_piece(game: &Game) {
    let shape = get_shape(game.current_type, game.current_rotation);
    let color = get_color(game.current_type);
    for &(dx, dy) in &shape {
        let x = game.pos.0 + dx;
        let y = game.pos.1 + dy;
        if x >= 0 && y >= 0 {
            draw_cell(x as f32, y as f32, color);
        }
    }
}

fn draw_grid_lines() {
    let line_color = Color::from_rgba(40, 40, 55, 255);
    for row in 0..=board::HEIGHT {
        let y = BOARD_OFFSET_Y + row as f32 * CELL_SIZE;
        draw_line(
            BOARD_OFFSET_X,
            y,
            BOARD_OFFSET_X + board::WIDTH as f32 * CELL_SIZE,
            y,
            1.0,
            line_color,
        );
    }
    for col in 0..=board::WIDTH {
        let x = BOARD_OFFSET_X + col as f32 * CELL_SIZE;
        draw_line(
            x,
            BOARD_OFFSET_Y,
            x,
            BOARD_OFFSET_Y + board::HEIGHT as f32 * CELL_SIZE,
            1.0,
            line_color,
        );
    }
}

fn draw_cell(col: f32, row: f32, color: Color) {
    let x = BOARD_OFFSET_X + col * CELL_SIZE;
    let y = BOARD_OFFSET_Y + row * CELL_SIZE;
    let border = 2.0;

    draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, color);

    let highlight = Color::from_rgba(
        ((color.r * 255.0) as u8).saturating_add(60),
        ((color.g * 255.0) as u8).saturating_add(60),
        ((color.b * 255.0) as u8).saturating_add(60),
        200,
    );
    draw_rectangle(x, y, CELL_SIZE, border, highlight);
    draw_rectangle(x, y, border, CELL_SIZE, highlight);

    let shadow = Color::from_rgba(
        ((color.r * 255.0) as u8).saturating_sub(80),
        ((color.g * 255.0) as u8).saturating_sub(80),
        ((color.b * 255.0) as u8).saturating_sub(80),
        200,
    );
    draw_rectangle(x, y + CELL_SIZE - border, CELL_SIZE, border, shadow);
    draw_rectangle(x + CELL_SIZE - border, y, border, CELL_SIZE, shadow);
}

fn draw_panel(game: &Game, highscores: &HighScores, font: &Font) {
    let text_color = WHITE;
    let label_color = Color::from_rgba(160, 160, 180, 255);
    let x = PANEL_X;
    let mut y = 40.0f32;

    draw_ui_text(font, "TETRIS", x, y, 28.0, text_color);
    y += 45.0;

    draw_ui_text(font, "SCORE", x, y, 16.0, label_color);
    y += 22.0;
    draw_ui_text(font, &game.score.to_string(), x, y, 24.0, text_color);
    y += 40.0;

    draw_ui_text(font, "LEVEL", x, y, 16.0, label_color);
    y += 22.0;
    draw_ui_text(font, &game.level.to_string(), x, y, 24.0, text_color);
    y += 40.0;

    draw_ui_text(font, "LINES", x, y, 16.0, label_color);
    y += 22.0;
    draw_ui_text(font, &game.lines_cleared.to_string(), x, y, 24.0, text_color);
    y += 50.0;

    draw_ui_text(font, "NEXT", x, y, 16.0, label_color);
    y += 25.0;

    let preview_x = x + 10.0;
    let preview_y = y;
    draw_rectangle(
        preview_x - 5.0,
        preview_y - 5.0,
        5.0 * CELL_SIZE,
        4.0 * CELL_SIZE,
        Color::from_rgba(30, 30, 45, 255),
    );

    let shape = get_shape(game.next_type, 0);
    let color = get_color(game.next_type);
    let min_cx = shape.iter().map(|&(cx, _)| cx).min().unwrap_or(0);
    let min_cy = shape.iter().map(|&(_, cy)| cy).min().unwrap_or(0);
    let max_cx = shape.iter().map(|&(cx, _)| cx).max().unwrap_or(0);
    let max_cy = shape.iter().map(|&(_, cy)| cy).max().unwrap_or(0);
    let pw = (max_cx - min_cx + 1) as f32;
    let ph = (max_cy - min_cy + 1) as f32;
    let offset_px = (5.0 - pw) / 2.0;
    let offset_py = (4.0 - ph) / 2.0;

    for &(dx, dy) in &shape {
        let px = preview_x + (dx - min_cx) as f32 * CELL_SIZE + offset_px * CELL_SIZE;
        let py = preview_y + (dy - min_cy) as f32 * CELL_SIZE + offset_py * CELL_SIZE;
        draw_rectangle(px + 1.0, py + 1.0, CELL_SIZE - 2.0, CELL_SIZE - 2.0, color);
    }

    y += 4.0 * CELL_SIZE + 20.0;
    draw_ui_text(font, "TOP 3", x, y, 14.0, label_color);
    y += 20.0;
    let entry_color = Color::from_rgba(200, 200, 120, 255);
    let score_right_edge = x + 140.0;
    for (i, entry) in highscores.entries().iter().enumerate() {
        let rank_text = format!("{}. {}", i + 1, entry.name);
        draw_ui_text(font, &rank_text, x, y, 12.0, entry_color);
        let score_text = entry.score.to_string();
        let dims = measure_ui_text(font, &score_text, 12.0);
        draw_ui_text(font, &score_text, score_right_edge - dims.width, y, 12.0, entry_color);
        y += 16.0;
    }
    y += 10.0;

    draw_ui_text(font, "CONTROLS", x, y, 14.0, label_color);
    y += 20.0;
    let controls = [
        ("\u{2190}\u{2192}  Move", 13.0),
        ("\u{2191}  Rotate", 13.0),
        ("\u{2193}  Soft Drop", 13.0),
        ("Space  Hard Drop", 13.0),
        ("P  Pause", 13.0),
        ("R  Restart", 13.0),
    ];
    for (text, size) in controls {
        draw_ui_text(font, text, x + 5.0, y, size, Color::from_rgba(120, 120, 140, 255));
        y += 18.0;
    }
}

fn draw_overlay(title: &str, subtitle: &str, font: &Font) {
    let bx = BOARD_OFFSET_X;
    let by = BOARD_OFFSET_Y + board::HEIGHT as f32 * CELL_SIZE / 2.0 - 40.0;
    let bw = board::WIDTH as f32 * CELL_SIZE;

    draw_rectangle(bx, by, bw, 80.0, Color::from_rgba(0, 0, 0, 180));

    let text_x = bx + bw / 2.0;
    let title_dimensions = measure_ui_text(font, title, 28.0);
    draw_ui_text(font, title, text_x - title_dimensions.width / 2.0, by + 40.0, 28.0, WHITE);

    let sub_dimensions = measure_ui_text(font, subtitle, 16.0);
    draw_ui_text(
        font,
        subtitle,
        text_x - sub_dimensions.width / 2.0,
        by + 65.0,
        16.0,
        Color::from_rgba(200, 200, 220, 255),
    );
}

fn draw_name_entry_overlay(name: &str, score: u32, font: &Font) {
    let bx = BOARD_OFFSET_X;
    let by = BOARD_OFFSET_Y + board::HEIGHT as f32 * CELL_SIZE / 2.0 - 60.0;
    let bw = board::WIDTH as f32 * CELL_SIZE;

    draw_rectangle(bx, by, bw, 130.0, Color::from_rgba(0, 0, 0, 200));

    let text_x = bx + bw / 2.0;
    let title = "NEW HIGH SCORE!";
    let title_dimensions = measure_ui_text(font, title, 24.0);
    draw_ui_text(
        font,
        title,
        text_x - title_dimensions.width / 2.0,
        by + 35.0,
        24.0,
        Color::from_rgba(255, 220, 100, 255),
    );

    let score_text = score.to_string();
    let score_dimensions = measure_ui_text(font, &score_text, 14.0);
    draw_ui_text(
        font,
        &score_text,
        text_x - score_dimensions.width / 2.0,
        by + 55.0,
        14.0,
        Color::from_rgba(200, 200, 220, 255),
    );

    let cursor = if get_time() as f64 % 1.0 < 0.5 { "_" } else { " " };
    let name_text = format!("{}{}", name, cursor);
    let name_dimensions = measure_ui_text(font, &name_text, 22.0);
    draw_ui_text(
        font,
        &name_text,
        text_x - name_dimensions.width / 2.0,
        by + 90.0,
        22.0,
        WHITE,
    );

    let hint = "Enter to Confirm";
    let hint_dimensions = measure_ui_text(font, hint, 12.0);
    draw_ui_text(
        font,
        hint,
        text_x - hint_dimensions.width / 2.0,
        by + 115.0,
        12.0,
        Color::from_rgba(160, 160, 180, 255),
    );
}
