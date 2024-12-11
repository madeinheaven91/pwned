use crate::shared::types::{Credential, Icon, InputMode};

pub struct App {
    pub mode: InputMode,
    pub credentials: Vec<Credential>, // all stored credentials
    pub hovered_cred_id: usize,
    pub hovered_field: usize,
    pub selected_cred_id: Option<usize>,
    pub search_query: String,
    pub filtered_credentials: Vec<Credential>,
}

impl App {
    pub fn new() -> Self {
        let cred = vec![
            Credential::new(
                0,
                "Gmail".to_owned(),
                Some(Icon::Mail),
                vec![
                    ("Username".to_owned(), "madeinheaven91".to_owned()),
                    ("Password".to_owned(), "secret_mail_pw".to_owned()),
                    ("Phone".to_owned(), "+7 982 743 23 32".to_owned()),
                ],
            ),
            Credential::new(
                1,
                "Github".to_owned(),
                Some(Icon::Github),
                vec![
                    ("Username".to_owned(), "madeinheaven91".to_owned()),
                    ("Password".to_owned(), "secret_github_pw".to_owned()),
                    ("Token".to_owned(), "iurfg2o8fg289f2bfy".to_owned()),
                ],
            ),
            Credential::new(
                2,
                "Tinkoff bank big money".to_owned(),
                Some(Icon::Bank),
                vec![("Password".to_owned(), "secret".to_owned())],
            ),
            Credential::new(
                3,
                "Whatassp chat".to_owned(),
                Some(Icon::Messenger),
                vec![("Password".to_owned(), "secret".to_owned())],
            ),
            Credential::new(
                4,
                "Gmail".to_owned(),
                Some(Icon::Mail),
                vec![("Password".to_owned(), "secret".to_owned())],
            ),
            Credential::new(
                5,
                "Github".to_owned(),
                Some(Icon::Github),
                vec![("Password".to_owned(), "secret".to_owned())],
            ),
            Credential::new(
                6,
                "Tinkoff bank big money".to_owned(),
                Some(Icon::Bank),
                vec![("Password".to_owned(), "secret".to_owned())],
            ),
            Credential::new(
                7,
                "Whatassp chat".to_owned(),
                Some(Icon::Messenger),
                vec![("Password".to_owned(), "secret".to_owned())],
            ),
        ];
        App {
            mode: InputMode::Normal,
            credentials: cred.clone(),
            hovered_cred_id: 0,
            hovered_field: 0,
            selected_cred_id: None,
            search_query: String::new(),
            filtered_credentials: cred,
        }
    }

    pub fn change_mode(&mut self, mode: InputMode) {
        self.mode = mode;
    }

    pub fn filter_credentials(&mut self) {
        self.filtered_credentials.clear();
        for cred in &self.credentials {
            if cred.title.to_lowercase().contains(&self.search_query) {
                self.filtered_credentials.push(cred.clone());
            }
        }
    }
}
