use crate::shared::types::InputMode;
use crossterm::event::{self, KeyCode};
use std::io::Error;

use crate::shared::app::App;


pub fn handle_selected(key: event::KeyEvent, state: &mut App) -> Result<(), Error> {
    match key.code {
        // выглядит как костыль
        KeyCode::Char('q') => return Err(Error::other("")),
        KeyCode::Char('s') => {
            state.change_mode(InputMode::Search)
        }
        KeyCode::Char('?') => {
            state.change_mode(InputMode::Help)
        }
        KeyCode::Left => {
            state.change_mode(InputMode::Normal)
        }
        KeyCode::Esc => {
            state.selected_cred_id = None;
            state.change_mode(InputMode::Normal)
        }
        _ => (),
    }
    Ok(())
}
