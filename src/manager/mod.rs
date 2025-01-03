use crypto::gen_salt;
use indexmap::IndexMap;
use rusqlite::Connection;
use std::{
    env::{self, set_current_dir}, fs::{self, exists}, io::Error, path::Path
};

use crate::shared::types::{credential::Credential, icon::Icon};
pub mod crypto;

pub struct Db {
    pub conn: Connection,
}

#[derive(Debug)]
pub struct DBEntry {
    pub id: usize,
    pub title: String,
    pub icon: i32,
}

#[derive(Debug)]
pub struct DBField {
    pub id: usize,
    pub entry_id: i32,
    pub key: String,
    pub value: Option<String>,
}

impl Db {
    pub fn new() -> Self {
        let conn = Connection::open("./pw.db").unwrap();
        Self { conn }
    }
    fn init(&self) -> anyhow::Result<()> {
        // Init tables
        let stmt = "CREATE TABLE IF NOT EXISTS Metadata (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );";
        self.conn.execute(stmt, [])?;
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

        let salt = gen_salt();
        let _ = self.insert_metadata("salt", &salt);

        Ok(())
    }
    pub fn insert_metadata(&self, key: &str, value: &str) -> anyhow::Result<()> {
        let stmt = format!(
            "INSERT INTO Metadata (key, value) VALUES ('{}', '{}');",
            key, value
        );
        self.conn.execute(&stmt, [])?;
        Ok(())
    }
    pub fn get_metadata(&self, key: &str) -> Result<String, Error> {
        let stmt = format!("SELECT value FROM Metadata WHERE key = '{}';", key);
        let mut stmt = self.conn.prepare(&stmt).unwrap();
        let value = stmt.query_row([], |r| {
            r.get(0)
        });
        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::other(format!("Failed to get metadata: {}",e))),

        }
    }
    pub fn get_creds(&self) -> anyhow::Result<IndexMap<usize, Credential>> {
        let mut stmt = self.conn.prepare("SELECT * FROM Entry;").unwrap();
        let entries = stmt.query_map([], |r| {
            Ok(DBEntry {
                id: r.get(0).unwrap(),
                title: r.get(1).unwrap(),
                icon: r.get(2).unwrap(),
            })
        })?;
        let e = entries.collect::<Result<Vec<DBEntry>, _>>()?;
        let mut creds = IndexMap::new();
        for entry in e {
            let s = format!("SELECT * FROM Field WHERE entry_id = {};", entry.id);
            let mut stmt = self.conn.prepare(&s).unwrap();
            let dbfields = stmt
                .query_map([], |r| {
                    Ok(DBField {
                        id: r.get(0).unwrap(),
                        entry_id: r.get(1).unwrap(),
                        key: r.get(2).unwrap(),
                        value: r.get(3).unwrap(),
                    })
                })
                .unwrap();
            let mut fields = vec![];
            for f in dbfields {
                let f = f.unwrap();

                fields.push((f.key, f.value.unwrap_or_default()));
            }
            let icon = Icon::from(entry.icon);
            let cred = Credential {
                id: entry.id,
                title: entry.title,
                icon: Some(icon),
                fields,
            };
            creds.insert(cred.id, cred);
        }
        Ok(creds)
    }

    pub fn insert_cred(&self, cred: &Credential) -> anyhow::Result<()> {
        Ok(())
    }
}

pub fn change_pwd() -> anyhow::Result<()> {
    let home = env::var("HOME").unwrap();
    let pwned = Path::new(&home).join(".pwned");
    set_current_dir(pwned)?;
    Ok(())
}

fn change_home() -> anyhow::Result<()> {
    let home = env::var("HOME").unwrap();
    let home = Path::new(&home);
    set_current_dir(home)?;
    Ok(())
}

fn init_pwd() -> anyhow::Result<()> {
    change_home()?;
    let exists = exists("./.pwned")?;
    if !exists {
        fs::create_dir_all("./.pwned")?
    };
    Ok(())
}

pub fn init_all() -> anyhow::Result<()> {
    init_pwd()?;
    change_pwd()?;
    let db = Db::new();
    db.init()?;
    let _ = db.conn.close();
    Ok(())
}
