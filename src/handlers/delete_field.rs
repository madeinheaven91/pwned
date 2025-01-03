use std::io::Error;
use crossterm::event::{self, KeyCode};

use crate::shared::{app::App, types::input_mode::InputMode};

pub fn handle_delete_field(key: event::KeyEvent, state: &mut App) -> Result<(), Error> {
    match key.code {
        KeyCode::Char('y') | KeyCode::Enter => {

            state.change_mode(InputMode::Normal);
        }
        _ => {

            state.change_mode(InputMode::Normal);
        }
    }
    Ok(())
}
