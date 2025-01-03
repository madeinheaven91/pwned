use crate::shared::{app::App, types::input_mode::InputMode, widgets::popup::Popup};
use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::Stylize,
    widgets::{Block, Paragraph},
    Frame,
};

pub fn delete_popup(f: &mut Frame, state: &mut App, area: Rect) {
    let outer = Popup::area_fixed(50, 5, area);
    let popup = Popup::default().block(Block::bordered().red());
    let inner = Popup::inner_area(&outer);

    let text = match state.mode {
        // FIXME: get text from lexicon
        InputMode::DeleteEntry => "Are you sure you want to delete this entry?",
        InputMode::DeleteField => "Are you sure you want to delete this field?",
        _ => panic!("something went wrong"),
    };

    let layout = Layout::vertical([
        Constraint::Length(1), // query
        Constraint::Length(1), // margin
        Constraint::Length(1), // options
    ])
    .split(inner);
    let options_layout =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .flex(Flex::SpaceBetween)
            .split(layout[2]);

    let text = Paragraph::new(text).alignment(Alignment::Center).red();
    let yes = Paragraph::new("<Y>").alignment(Alignment::Center).red();
    let no = Paragraph::new("<N>").alignment(Alignment::Center).red();

    f.render_widget(popup, outer);
    f.render_widget(text, layout[0]);
    f.render_widget(yes, options_layout[0]);
    f.render_widget(no, options_layout[1]);
}
