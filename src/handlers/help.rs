use crate::shared::types::InputMode;
use crate::App;
use crossterm::event::{self, KeyCode};
use std::io::Error;

pub fn handle_help(key: event::KeyEvent, state: &mut App) -> Result<(), Error> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('?') => {
            state.change_mode(InputMode::Normal)
        }
        _ => (),
    }
    Ok(())
}
