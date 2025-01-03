use ratatui::{
    layout::{Alignment, Rect}, style::Stylize, widgets::Paragraph, Frame
};

use crate::shared::{app::App, types::input_mode::InputMode, utils::Lexicon};

pub fn helper(f: &mut Frame, state: &mut App, area: Rect) {
    let text = match state.mode {
        InputMode::Normal => Lexicon::HelperNormal.str(),
        InputMode::Selected => Lexicon::HelperSelected.str(),
        InputMode::Search => Lexicon::HelperSearch.str(),
        _ => ""
    };
    let helper = Paragraph::new(
        text
    ).white().alignment(Alignment::Center);
    f.render_widget(helper, area)
}
