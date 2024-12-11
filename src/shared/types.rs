use core::fmt;
use derive_setters::Setters;
use ratatui::{buffer::Buffer, layout::Rect, style::Style, text::{Line, Text}, widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap}};
use std::fmt::Display;

#[derive(Default, Clone, Copy, Debug)]
pub enum Icon {
    #[default]
    None,
    Mail,
    Github,
    Bank,
    Messenger,
}

impl Icon {
    pub fn get(self) -> &'static str {
        match self {
            Icon::None => "",
            Icon::Mail => "",
            Icon::Github => "",
            Icon::Bank => "󰁰",
            Icon::Messenger => "󰵅",
        }
    }
    pub fn get_art(self) -> &'static str {
        match self {
            Icon::None => "",
            Icon::Github => {
                "      @@@@@@@@      
   @@@@@@@@@@@@@@   
 @@@@  @@  @@  @@@@ 
 @@@@          @@@@ 
@@@@            @@@@
@@@@            @@@@
@@@@@          @@@@@
 @@ @@@@    @@@@@@@ 
  @@@       @@@@@@  
     @@@    @@@     "
            }

            Icon::Mail => {
"@@@@@@@@@@@@@@@@@@@@
@  @            @  @
@    @        @    @
@      @    @      @
@    @   @@   @    @
@  @            @  @
@@................@@"
            }
            _ => "",
        }
    }
}

impl Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

pub enum InputMode {
    Normal,
    Help,
    Search,
    Selected,
}

#[derive(Clone, Default)]
pub struct Credential {
    pub id: usize,
    pub title: String,
    pub icon: Option<Icon>,
    pub fields: Vec<(String, String)>,
}

impl Credential {
    pub fn new(
        id: usize,
        title: String,
        icon: Option<Icon>,
        fields: Vec<(String, String)>,
    ) -> Self {
        Credential {
            id,
            title,
            icon,
            fields,
        }
    }
}

// FIXME: handle this shit
#[derive(Debug, Default, Setters)]
pub struct Popup<'a> {
    #[setters(into)]
    title: Line<'a>,
    #[setters(into)]
    content: Text<'a>,
    border_style: Style,
    title_style: Style,
    style: Style,
}

impl Widget for Popup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // ensure that all cells under the popup are cleared to avoid leaking content
        Clear.render(area, buf);
        let block = Block::new()
            .title(self.title)
            .title_style(self.title_style)
            .borders(Borders::ALL)
            .border_style(self.border_style);
        Paragraph::new(self.content)
            .wrap(Wrap{ trim: true })
            .style(self.style)
            .block(block)
            .render(area, buf);
    }
}
