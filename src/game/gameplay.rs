use crate::game::field::draw_field;
use crate::game::gamestate::GameState;
use crossterm::{cursor::MoveTo, execute, queue, style::Print};
use std::io::{Result, stdout};

pub fn render(state: &GameState) -> Result<()> {
    let mut stdout = stdout();

    draw_field(&state);

    for (x, y) in state.snake.clone() {
        execute!(stdout, MoveTo(x, y), Print("*"));
    }

    Ok(())
}
