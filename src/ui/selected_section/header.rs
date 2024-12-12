use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect}, style::Stylize, widgets::{Block, Padding, Paragraph}, Frame
};

use crate::shared::{app::App, types::input_mode::InputMode};

pub fn header(f: &mut Frame, state: &mut App, area: Rect) {
    let (_, icon_w) = state
        .selected_cred
        .clone()
        .unwrap_or_default()
        .icon
        .unwrap_or_default()
        .size();
    let pad = match area.height % 2{
        0 => {
            area.height / 2 - 2
        }
        _ => {
            area.height / 2 - 1
        }
    };
    let areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(icon_w), Constraint::Length(3), Constraint::Min(3)])
        .split(area);
    let icon_area = areas[0];
    let title_area = areas[2];

    let mut icon = Paragraph::new(
        state
            .selected_cred
            .as_ref()
            .unwrap()
            .icon
            .unwrap_or_default()
            .get_art(),
        )
        .white()
        .alignment(Alignment::Center);
    let mut title = Paragraph::new(
        state
            .selected_cred
            .as_ref()
            .unwrap()
            .title
            .clone()
        )
        .white()
        .alignment(Alignment::Center)
        .block(Block::bordered().padding(Padding::top(pad)));
    
    if state.hovered_field.unwrap() == 0 && state.mode == InputMode::Selected {
        match state.hovered_subfield {
            0 => { icon = icon.black().on_yellow(); title = title.yellow()}
            _ => { icon = icon.yellow(); title = title.black().on_yellow()}
        }
    }

    f.render_widget(icon, icon_area);
    f.render_widget(title, title_area)
}
