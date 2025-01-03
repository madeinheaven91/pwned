use indexmap::IndexMap;

use crate::{
    manager::Db,
    shared::{types::{credential::Credential, icon::Icon, input_mode::InputMode}, utils::safe_sub},
};

pub struct App {
    pub mode: InputMode,
    pub search_query: String,
    pub credentials: IndexMap<usize, Credential>, // all stored credentials
    pub hovered_cred_id: usize,
    pub selected_cred: Option<Credential>,
    pub filtered_credentials: Vec<Credential>,
    pub hovered_field: Option<usize>,
    pub hovered_subfield: usize,
    pub db: Db,
    pub master: String,
    pub salt: String
}

impl App {
    pub fn new(master: String) -> Self {
        let mut cred = IndexMap::new();
        cred.insert(0, Credential::new(0, "Github".to_string(), Some(Icon::Github), vec![
            ("Username".to_owned(), "heven91".to_owned()),
            ("Password".to_owned(), "secret".to_owned()),
            ("Phone".to_owned(), "123235628".to_owned()),
        ]));
        cred.insert(1, Credential::new(1, "Gmail".to_string(), Some(Icon::Mail), vec![
            ("Username".to_owned(), "heven91".to_owned()),
            ("Password".to_owned(), "secret".to_owned()),
        ]));
        cred.insert(2, Credential::new(2, "Pornhub".to_string(), None, vec![
            ("Username".to_owned(), "dro4er".to_owned()),
            ("Password".to_owned(), "koncha228".to_owned()),
        ]));

        let db = Db::new();
        App {
            mode: InputMode::Normal,
            credentials: cred.clone(),
            hovered_cred_id: 0,
            hovered_field: None,
            hovered_subfield: 0,
            selected_cred: None,
            search_query: String::new(),
            filtered_credentials: cred.values().cloned().collect(),
            master,
            salt: db.get_metadata("salt").unwrap(),
            db
        }
    }

    pub fn change_mode(&mut self, mode: InputMode) {
        self.mode = mode;
    }

    pub fn filter_credentials(&mut self) {
        self.filtered_credentials.clear();
        for cred in self.credentials.values(){
            if cred.title.to_lowercase().contains(&self.search_query) {
                self.filtered_credentials.push(cred.clone());
            }
        }
    }

    pub fn add_cred(&mut self){
        unimplemented!()
    }
    
    pub fn delete_hovered_cred(&mut self){
        self
            .credentials
            .shift_remove(&self.filtered_credentials[self.hovered_cred_id].id.clone());
        self.selected_cred = None;
        self.hovered_cred_id = safe_sub(self.hovered_cred_id, 0);
        self.hovered_subfield = 0;
        self.hovered_field = None;
        self.filter_credentials();
    }
}
