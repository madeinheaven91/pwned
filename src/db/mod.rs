use rusqlite::Connection;

pub struct Db{
    pub conn: Connection
}

#[derive(Debug)]
pub struct DBEntry {
    pub id: i32,
    pub title: String,
    pub icon: i32
}

#[derive(Debug)]
pub struct DBField {
    pub id: i32,
    pub entry_id: i32,
    pub key: String,
    pub value: Option<String>
}

impl Db {
    pub fn new() -> Self {
        let conn = Connection::open("credible.db").unwrap();
        Self { conn }
    }
    pub fn init(&self) -> anyhow::Result<()> {
        let stmt = "CREATE TABLE IF NOT EXISTS Entry (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            icon INTEGER DEFAULT 0
        );";
        self.conn.execute(stmt, [])?;
        let stmt = "CREATE TABLE IF NOT EXISTS Field(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                entry_id INTEGER NOT NULL references Entry(id),
                key TEXT NOT NULL,
                value TEXT,
                FOREIGN KEY(entry_id) REFERENCES Entry(id)
            );";
        self.conn.execute(stmt, [])?;

        Ok(())
    }
}

