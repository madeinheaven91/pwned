mod search_bar;
mod credentials;

use crate::{shared::app::App, ui::list_section::credentials::{no_entries, nothing_found}};
use crate::ui::list_section::{
        credentials::entries,
        search_bar::search_bar,
    };
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub fn list_section(f: &mut Frame, state: &mut App, area: Rect) {
    let list_section_chunk = Layout::default()
        .margin(1)
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(1), Constraint::Min(10)])
        .split(area);

    let search_area = list_section_chunk[0];
    let entries_area = list_section_chunk[2];

    search_bar(f, state, search_area);

    if state.credentials.is_empty(){
        no_entries(f, state, entries_area)
    }
    else if state.filtered_credentials.is_empty(){
        nothing_found(f, state, entries_area) 
    }
    else{
        entries(f, state, entries_area);
    }
}
