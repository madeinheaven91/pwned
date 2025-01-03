use std::{env, io::{self, stdout, Write}};

use crate::{manager::Db, shared::app::App};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use manager::{change_pwd, crypto::{self, check_hash}, init_all};
use ratatui::{
    backend::{Backend, CrosstermBackend}, Terminal
};

mod handlers;
mod ui;
mod shared;
mod manager;

use handlers::handle_keys;
use ui::ui;

fn main() -> anyhow::Result<()> {
    init_all()?;
    change_pwd()?;
    println!("{}", env::var("PWD").unwrap());
    let db = Db::new();
    let salt = db.get_metadata("salt").unwrap();
    let mut master = String::new();
    let master_hash = db.get_metadata("master");
    match master_hash {
        Err(_) => {
            // First time
            let mut repeat = String::new();
            print!("Select master password: ");
            stdout().flush().unwrap();
            io::stdin().read_line(&mut master).unwrap();
            print!("Repeat: ");
            stdout().flush().unwrap();
            io::stdin().read_line(&mut repeat).unwrap();
            if master != repeat {

                println!("Passwords don't match!");
                return Ok(());
            }else{
                let master_hash = crypto::hash(&master, &master, &salt);
                db.insert_metadata("master", &master_hash)?;
            }
        },
        Ok(hash) => {
            // Other times
            print!("Enter master password: ");
            stdout().flush().unwrap();
            io::stdin().read_line(&mut master).unwrap();
            match check_hash(&master, &hash, &salt) {
                true => (),
                false => {
                    panic!("Wrong password")
                },
            }
        }
    }
    let mut state = App::new(master);

    enable_raw_mode()?;
    execute!(
        std::io::stdout(), 
        EnterAlternateScreen, 
        EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

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
        if state.selected_cred.is_none(){
            state.hovered_field = None;
        }
        terminal.draw(|f| ui(f, state))?;
        if let Event::Key(key) = event::read()? {
            handle_keys(key, state)?;
        }
    }
}
