use std::io::Error;
use crossterm::event::{self};

use crate::shared::{app::App, types::input_mode::InputMode};

pub fn handle_edit(key: event::KeyEvent, state: &mut App) -> Result<(), Error> {
    use event::KeyCode;
    match key.code {
        KeyCode::Esc => state.change_mode(InputMode::Normal),
        KeyCode::Enter => state.change_mode(InputMode::Normal),
        _ => state.change_mode(InputMode::Normal)
    }
    Ok(())
}
