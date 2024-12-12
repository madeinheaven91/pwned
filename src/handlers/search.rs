use crate::shared::{app::App, types::input_mode::InputMode};
use std::io::Error;
use crossterm::event::{self, KeyCode};

pub fn handle_search(key: event::KeyEvent, state: &mut App) -> Result<(), Error> {
    // execute!(
    //     std::io::stdout(),
    //     Show
    // )?;
    match key.code {
        KeyCode::Esc => {
            state.search_query = String::new();
            state.filter_credentials();
            state.change_mode(InputMode::Normal)
        }
        KeyCode::Enter => {
            state.change_mode(InputMode::Normal)
        }
        KeyCode::Delete => {
            state.search_query = String::new();
            state.filter_credentials()
        }
        KeyCode::Char(c) => {
            state.search_query.push(c);
            state.filter_credentials()
        }
        KeyCode::Backspace => {
            state.search_query.pop();
            state.filter_credentials()
        }
        _ => ()
    }
    Ok(())
}
