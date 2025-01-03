use crossterm::event::{self, KeyCode};
use std::io::Error;

use crate::shared::{
    app::App,
    types::{credential::Credential, input_mode::InputMode},
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
            if state.filtered_credentials.is_empty() {
                return Ok(());
            }
            state.selected_cred = Some(
                state
                    .credentials
                    .get(&state.filtered_credentials[state.hovered_cred_id].id)
                    .unwrap()
                    .clone(),
            );
            state.hovered_field = Some(0);
            state.change_mode(InputMode::Selected)
        }
        KeyCode::Char('d') => {
            state.change_mode(InputMode::DeleteEntry)
        }
        KeyCode::Char('n') => {
            let new_cred = Credential::new(69, "Bebra".to_owned(), None, vec![]);
            state.credentials.insert(new_cred.id, new_cred);
            state.filter_credentials();
        }
        _ => (),
    }
    Ok(())
}
