use core::fmt::{self, Display};

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
    pub fn vec(&self) -> Vec<&'static str> {
        match self {
            Icon::Github => vec![
"      @@@@@@@@      ",
"   @@@@@@@@@@@@@@   ",
" @@@@  @@  @@  @@@@ ",
" @@@@          @@@@ ",
"@@@@            @@@@",
"@@@@            @@@@",
"@@@@@          @@@@@",
" @@ @@@@    @@@@@@@ ",
"  @@@       @@@@@@  ",
"     @@@    @@@     "],

            Icon::Mail => vec![
"@@@@@@@@@@@@@@@@@@@@",
"@  @            @  @",
"@    @        @    @",
"@      @    @      @",
"@    @   @@   @    @",
"@  @            @  @",
"@@................@@"],
            _ => vec![]
        }
    }
    pub fn get_art(&self) -> String {
        self.vec().join("\n")
    }

    pub fn size(&self) -> (u16, u16){
        let mut lengths: Vec<u8> = vec![];
        let _ = self.vec().iter().map(|x| lengths.push(x.len() as u8)).collect::<Vec<()>>();
        let h = self.vec().len() as u16;
        let w = lengths.into_iter().max().unwrap_or_default() as u16;
        (h, w)
    }
}

impl Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}
