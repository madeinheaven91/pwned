
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::shared::app::App;

pub fn header(f: &mut Frame, state: &mut App, area: Rect) {
    let areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);
    let icon_area = areas[0];
    let title_area = areas[1];

    let icon = Paragraph::new(
        state.credentials[state.selected_cred_id.unwrap()]
            .icon
            .unwrap_or_default()
            .get_art(),
    )
    .alignment(Alignment::Center);
    let title = Paragraph::new(
        state.credentials[state.selected_cred_id.unwrap()]
            .title
            .clone(),
    )
    .alignment(Alignment::Center)
    .block(Block::bordered());

    f.render_widget(icon, icon_area);
    f.render_widget(title, title_area)
}
