use crate::{shared::types::input_mode::InputMode, App};
use std::io::Error;
use crossterm::event::{self};
use crate::handlers::{help::handle_help, normal::handle_normal, search::handle_search, selected::handle_selected};

mod normal;
mod search;
mod help;
mod selected;

pub fn handle_keys(key: event::KeyEvent, state: &mut App) -> Result<(), Error> {
    match state.mode {
        InputMode::Normal => {
            handle_normal(key, state)?
        }
        InputMode::Help => {
            handle_help(key, state)?
        }
        InputMode::Search => {
            handle_search(key, state)?
        }
        InputMode::Selected => {
            handle_selected(key, state)?
        }
    }
    Ok(())
}
