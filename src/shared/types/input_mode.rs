#[derive(Debug, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Help,
    Search,
    Selected,
    Edit,
    DeleteEntry,
    DeleteField
}
