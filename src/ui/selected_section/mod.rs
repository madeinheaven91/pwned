use crate::{
    shared::app::App, ui::selected_section::{entry::entry, header::header, helper::helper}
};
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout, Rect},
    Frame,
};

mod entry;
mod header;
mod helper;

pub fn selected_section(f: &mut Frame, state: &mut App, area: Rect) {
    if state.selected_cred.is_none() {
        return;
    }

    let (icon_h, _) = state
        .selected_cred
        .clone()
        .unwrap_or_default()
        .icon
        .unwrap_or_default()
        .size();

    let main_chunk = Layout::default()
        .margin(2)
        .direction(Direction::Vertical)
        .flex(Flex::SpaceBetween)
        .constraints([
            Constraint::Length(icon_h),
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(1),
        ])
        .split(area);
    let header_area = main_chunk[0];
    let entry_area = main_chunk[2];
    let helper_area = main_chunk[3];

    header(f, state, header_area);
    entry(f, state, entry_area);
    helper(f, state, helper_area);
}
