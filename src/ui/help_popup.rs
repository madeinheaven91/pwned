use ratatui::{layout::{Alignment, Constraint, Flex, Layout, Rect}, style::{Style, Stylize}, widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget}, Frame};

use crate::shared::{app::App, types::Popup};


pub fn help_popup(f: &mut Frame, state: &mut App, area: Rect) {
    // help popup
    let [help] = Layout::horizontal([Constraint::Length(area.width / 2 as u16)])
        .flex(Flex::Center)
        .areas(f.area());
    let [help] = Layout::vertical([Constraint::Length(area.height / 2 as u16)])
        .flex(Flex::Center)
        .areas(help);
    // let help_msg = Paragraph::new("bebra").alignment(Alignment::Center);
    // let help_box = Block::bordered().title("Help");
    // f.render_widget(help_msg.block(help_box), help);

    let popup = Popup::default()
            .content("(help msg here)") // TODO:
            .style(Style::default().red());
    f.render_widget(popup, help);
}
