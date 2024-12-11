use crate::shared::{app::App, utils::Lexicon};
use ratatui::{
    layout::Rect,
    widgets::{Block, BorderType, Padding, Paragraph},
    Frame,
};
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    prelude::Stylize,
};

pub fn entries(f: &mut Frame, state: &mut App, area: Rect) {
    let mut constrains: Vec<Constraint> = vec![];
    for _ in &state.filtered_credentials {
        constrains.push(Constraint::Length(3));
    }

    let entries_chunk = Layout::default().constraints(constrains).split(area);

    for (i, item) in state.filtered_credentials.iter().enumerate() {
        let text = Paragraph::new(format!(
            "{}   {}",
            item.icon.unwrap_or_default(),
            item.title
        ));
        let block = if i == state.hovered_cred_id {
            Block::bordered()
                .padding(Padding::new(1, 1, 0, 0))
                .border_type(BorderType::Thick)
                .green()
        } else {
            Block::bordered().padding(Padding::new(1, 1, 0, 0)).white()
        };

        let entry = text.block(block);
        f.render_widget(entry, entries_chunk[i]);
    }
}

pub fn no_entries(f: &mut Frame, _state: &mut App, area: Rect) {
    let not_found_area = Layout::default()
        .constraints([Constraint::Length(3)])
        .split(area);
    f.render_widget(
        Paragraph::new(Lexicon::NoEntries.str()).alignment(Alignment::Center),
        not_found_area[0],
    );
}

pub fn nothing_found(f: &mut Frame, _state: &mut App, area: Rect) {
    let not_found_area = Layout::default()
        .constraints([Constraint::Length(3)])
        .split(area);
    f.render_widget(
        Paragraph::new(Lexicon::NothingFound.str()).alignment(Alignment::Center),
        not_found_area[0],
    );
}
