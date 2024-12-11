use crate::{ui::selected_section::header::header, App};
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout, Rect},
    widgets::{Block, Padding, Paragraph},
    Frame,
};

mod header;

pub fn selected_section(f: &mut Frame, state: &mut App, area: Rect) {
    if state.selected_cred_id.is_none() {
        return;
    }

    let selected = state.filtered_credentials[state.selected_cred_id.unwrap_or_default()].clone();
    let mut field_constraints = vec![Constraint::Length(3)];
    for _ in selected.fields.iter() {
        field_constraints.push(Constraint::Length(3));
    }

    let main_chunk = Layout::default()
        .margin(2)
        .direction(Direction::Vertical)
        .flex(Flex::SpaceBetween)
        .constraints([Constraint::Length(14), Constraint::Min(10)])
        .split(area);

    let entry_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(field_constraints)
        .split(main_chunk[1]);

    for (i, entry) in selected.fields.iter().enumerate() {
        let entry_segments = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(entry_chunk[i]);
        let key_text = Paragraph::new(entry.0.to_string());
        let value_text = Paragraph::new(entry.1.to_string());
        let block = Block::bordered().padding(Padding::new(1, 1, 0, 0));
        f.render_widget(key_text.block(block.clone()), entry_segments[0]);
        f.render_widget(value_text.block(block), entry_segments[1]);
    }

    header(f, state, main_chunk[0])
}
