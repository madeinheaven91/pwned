use crate::shared::types::icon::Icon;


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
