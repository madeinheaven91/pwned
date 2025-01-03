use crossterm::event::{self, KeyCode};
use std::io::Error;

use crate::shared::{app::App, types::input_mode::InputMode};

pub fn handle_delete_entry(key: event::KeyEvent, state: &mut App) -> Result<(), Error> {
    match key.code {
        KeyCode::Char('y') | KeyCode::Enter => {
            if state.filtered_credentials.is_empty() {
                state.change_mode(InputMode::Normal);
            }
            state.delete_hovered_cred();
            state.change_mode(InputMode::Normal);
        }
        _ => {
            state.change_mode(InputMode::Normal);
        }
    }
    Ok(())
}
