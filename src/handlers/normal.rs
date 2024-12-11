use std::io::Error;
use crossterm::event::{self, KeyCode};

use crate::shared::{app::App, types::InputMode, utils::{safe_add, safe_sub}};


pub fn handle_normal(key: event::KeyEvent, state: &mut App) -> Result<(), Error> {
    match key.code {
        // выглядит как костыль
        KeyCode::Char('q') => return Err(Error::other("")),
        KeyCode::Char('s') => {
            state.change_mode(InputMode::Search)
        }
        KeyCode::Char('?') => {
            state.change_mode(InputMode::Help)
        }
        KeyCode::Tab | KeyCode::Down => {
            state.hovered_cred_id = safe_add(state.hovered_cred_id, state.filtered_credentials.len() - 1);
        }
        KeyCode::BackTab | KeyCode::Up => {
            state.hovered_cred_id = safe_sub(state.hovered_cred_id, 0);
        }
        KeyCode::Enter | KeyCode::Right => {
            state.selected_cred_id = Some(state.hovered_cred_id);
            state.change_mode(InputMode::Selected)
        }
        _ => (),
    }
    Ok(())
}
