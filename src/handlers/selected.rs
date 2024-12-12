use crossterm::event::{self, KeyCode};
use std::io::Error;

use crate::shared::{app::App, types::input_mode::InputMode, utils::{safe_add, safe_sub}};

pub fn handle_selected(key: event::KeyEvent, state: &mut App) -> Result<(), Error> {
    match key.code {
        // выглядит как костыль
        KeyCode::Char('q') => return Err(Error::other("")),
        KeyCode::Char('s') => state.change_mode(InputMode::Search),
        KeyCode::Char('?') => state.change_mode(InputMode::Help),
        KeyCode::Char('j') | KeyCode::Down => {
            state.hovered_field = Some(safe_add(
                state.hovered_field.unwrap_or_default(),
                state.selected_cred.as_ref().unwrap().fields.len(),
            ))
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.hovered_field = Some(safe_sub(
                state.hovered_field.unwrap_or_default(),
                0
            ))
        }
        KeyCode::Left | KeyCode::Char('h') => {
            if state.hovered_subfield == 0 {
                state.change_mode(InputMode::Normal)
            }else{
                state.hovered_subfield = 0;
            }
        }
        KeyCode::Right | KeyCode::Char('l') => {
            if state.hovered_subfield == 0 {
                state.hovered_subfield = 1;
            }else{
                state.hovered_subfield = 0;
            }
        }
        KeyCode::Esc => {
            state.selected_cred = None;
            state.change_mode(InputMode::Normal)
        }
        _ => (),
    }
    Ok(())
}
