use ratatui::{
    layout::{Constraint, Direction, Layout, Rect}, style::Stylize, widgets::{Block, Padding, Paragraph}, Frame
};

use crate::shared::{app::App, types::input_mode::InputMode};

pub fn entry(f: &mut Frame, state: &mut App, area: Rect) {
    let selected = state
        .selected_cred
        .as_ref()
        .unwrap()
        .clone();
    let mut field_constraints = vec![Constraint::Length(3)];
    for _ in selected.fields.iter() {
        field_constraints.push(Constraint::Length(3));
    }

    let entry_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(field_constraints)
        .split(area);

    for (i, entry) in selected.fields.iter().enumerate() {
        let entry_segments = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(entry_chunk[i]);
        let mut key_text = Paragraph::new(entry.0.to_string()).white();
        let mut value_text = Paragraph::new(entry.1.to_string()).white();
        let block = Block::bordered().padding(Padding::new(1, 1, 0, 0));

        if state.hovered_field.unwrap() == i + 1 && state.mode == InputMode::Selected {
            match state.hovered_subfield{
                0 => {
                    key_text = key_text.black().on_yellow();
                    value_text = value_text.yellow();
                }
                _ => {
                    key_text = key_text.yellow();
                    value_text = value_text.black().on_yellow();
                }
            }
        }
        
        let key = key_text.block(block.clone());
        let value = value_text.block(block);
        f.render_widget(key, entry_segments[0]);
        f.render_widget(value, entry_segments[1]);
    }
}
