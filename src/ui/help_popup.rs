use ratatui::layout::{Alignment, Constraint, Flex, Layout, Rect};
use ratatui::{
    style::Stylize,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::shared::{app::App, widgets::popup::Popup};

pub fn help_popup(f: &mut Frame, _state: &mut App, area: Rect) {
    let outer = Popup::area(0.5, 0.8, area);
    let inner = Popup::inner_area(&outer);

    let layout = Layout::vertical([
        Constraint::Length(1), // title
        Constraint::Length(1), // margin
        Constraint::Min(3), // content
        Constraint::Length(1), // margin
        Constraint::Length(1), // footer
    ])
    .flex(Flex::SpaceBetween)
    .split(inner);

    let popup = Popup::default().block(Block::bordered());
    let title = Paragraph::new("Keybindings").alignment(Alignment::Center);
    let footer = Paragraph::new("made in heaven. 2025 - 2025").alignment(Alignment::Center).dim();
    let content_area = Layout::horizontal([
        Constraint::Percentage(45),
        Constraint::Percentage(10),
        Constraint::Percentage(45)
    ])
    .split(layout[2]);
    // FIXME: get content text from lexicon
    let content_keys = Paragraph::new(
        "Quit\nToggle help"
    )
        .alignment(Alignment::Right)
        .blue();
    let content_values = Paragraph::new(
        "<Q>\n<?>"
    )
        .alignment(Alignment::Left)
        .green();


    f.render_widget(popup, outer);
    f.render_widget(title, layout[0]);
    f.render_widget(content_keys, content_area[0]);
    f.render_widget(content_values, content_area[2]);
    f.render_widget(footer, layout[4]);
}
