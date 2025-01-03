use crate::{handlers::{delete_entry::handle_delete_entry, delete_field::handle_delete_field, edit::handle_edit}, shared::{app::App, types::input_mode::InputMode}};
use std::io::Error;
use crossterm::event::{self};
use crate::handlers::{help::handle_help, normal::handle_normal, search::handle_search, selected::handle_selected};

mod normal;
mod search;
mod help;
mod selected;
mod edit;
mod delete_entry;
mod delete_field;

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
        InputMode::Edit => {
            handle_edit(key, state)?
        }
        InputMode::DeleteEntry => {
            handle_delete_entry(key, state)?
        }
        InputMode::DeleteField => {
            handle_delete_field(key, state)?
        }
    }
    Ok(())
}
