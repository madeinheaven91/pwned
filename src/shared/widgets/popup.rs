use derive_setters::Setters;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, Clear, Paragraph, Widget},
};

// NOTE: Popup is just a clear centered area with a border
#[allow(clippy::needless_update)]
#[derive(Debug, Default, Setters)]
pub struct Popup<'a> {
    block: Block<'a>,
}

impl Popup<'_> {
    pub fn area(horizontal_percent: f32, vertical_percent: f32, area: Rect) -> Rect {
        let [rect] = Layout::horizontal([Constraint::Max(
            (area.width as f32 * horizontal_percent) as u16,
        )])
        .flex(Flex::Center)
        .areas(area);
        let [rect] = Layout::vertical([Constraint::Max(
            (area.height as f32 * vertical_percent) as u16,
        )])
        .flex(Flex::Center)
        .areas(rect);
        rect
    }
    pub fn area_fixed(horizontal: u16, vertical: u16, area: Rect) -> Rect {
        let [rect] = Layout::horizontal([Constraint::Length(horizontal)])
            .flex(Flex::Center)
            .areas(area);
        let [rect] = Layout::vertical([Constraint::Length(vertical)])
            .flex(Flex::Center)
            .areas(rect);
        rect
    }
    pub fn inner_area(area: &Rect) -> Rect {
        Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width - 2,
            height: area.height - 2,
        }
    }
}

impl Widget for Popup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        // FIXME: костыль слегка
        Paragraph::new("").block(self.block).render(area, buf);
    }
}
