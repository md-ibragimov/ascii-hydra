use crate::game::gamestate::GameState;
use crossterm::{cursor::MoveTo, execute, queue, style::Print};
use std::io::{Result, stdout};

pub fn render(state: &GameState) -> Result<()> {
    let mut stdout = stdout();

    for (x, y) in state.snake.clone() {
        execute!(stdout, MoveTo(x, y), Print("*"))?;
    }

    if let Some((x, y)) = state.tail_to_clear {
        execute!(stdout, MoveTo(x, y), Print(" "))?;
    }

    Ok(())
}
