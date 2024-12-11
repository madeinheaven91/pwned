use crate::shared::app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{
    backend::{Backend, CrosstermBackend}, Terminal
};
use std::io::Error;

mod handlers;
mod ui;
mod shared;

use handlers::handle_keys;
use ui::ui;



fn main() -> Result<(), Error> {
    enable_raw_mode()?;
    execute!(
        std::io::stdout(), 
        EnterAlternateScreen, 
        EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut state = App::new();

    let result = run_app(&mut terminal, &mut state);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    if let Err(err) = result {
        if !err.to_string().is_empty() {
            println!("{}", err)
        }
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, state: &mut App) -> Result<(), std::io::Error> {
    loop {
        if state.filtered_credentials.is_empty(){
            state.selected_cred_id = None;
        }        terminal.draw(|f| ui::<B>(f, state))?;
        if let Event::Key(key) = event::read()? {
            handle_keys(key, state)?;
        }
    }
}
