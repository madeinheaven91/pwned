use ratatui::{layout::Rect, widgets::{Block, BorderType, Padding, Paragraph}, Frame};
use ratatui::prelude::Stylize;

use crate::shared::{app::App, types::input_mode::InputMode, utils::Lexicon};


pub fn search_bar(f: &mut Frame, state: &mut App, area: Rect) {
    let mut border = Block::bordered()
        .border_type(BorderType::Rounded)
        .padding(Padding::new(1, 1, 0, 0));
    let text: Paragraph;

    match state.mode{
        InputMode::Search => {
            border = border.green();
            text = Paragraph::new(state.search_query.clone())
        }
        _ => {
            border =  border.white();
            text = if state.search_query.is_empty() {
                Paragraph::new(Lexicon::SearchPlaceholder.str()).blue()
            } else {
                Paragraph::new(state.search_query.clone())
            };
        }
    };

    f.render_widget(text.block(border), area)
}
