use crossterm::event::{self, KeyCode};
use std::io::Error;

use crate::shared::{
    app::App,
    types::input_mode::InputMode,
    utils::{safe_add, safe_sub},
};

pub fn handle_normal(key: event::KeyEvent, state: &mut App) -> Result<(), Error> {
    match key.code {
        // выглядит как костыль
        KeyCode::Char('q') => return Err(Error::other("")),
        KeyCode::Char('s') => state.change_mode(InputMode::Search),
        KeyCode::Char('?') => state.change_mode(InputMode::Help),
        KeyCode::Tab | KeyCode::Down | KeyCode::Char('j') => {
            state.hovered_cred_id = safe_add(state.hovered_cred_id, state.filtered_credentials.len() - 1);
        }
        KeyCode::BackTab | KeyCode::Up | KeyCode::Char('k') => {
            state.hovered_cred_id = safe_sub(state.hovered_cred_id, 0);
        }
        KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
            state.selected_cred = Some(
                state
                    .credentials
                    .get(&state.hovered_cred_id)
                    .unwrap()
                    .clone(),
            );
            state.hovered_field = Some(0);
            state.change_mode(InputMode::Selected)
        }
        _ => (),
    }
    Ok(())
}
