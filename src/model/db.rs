use rusqlite::{Connection, Result};
use crate::model::notes::Note;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    pub fn init(&self) -> Result<()> {
        self.conn.execute(
            "
            CREATE TABLE IF NOT EXISTS Notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add_note(&self, name: &str, content: &str) -> Result<usize> {
        self.conn.execute(
            "INSERT INTO Notes (name, content) VALUES (?1, ?2)",
            (&name, &content)
        )
    }

    pub fn delete_note(&self, index: u32) -> Result<usize> {
        match self.conn.execute("DELETE FROM Notes WHERE id = ?1", &[&index]) {
            Ok(num) => {
                if num > 0 {
                    Ok(num)
                } else {
                    Err(rusqlite::Error::QueryReturnedNoRows)
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_all_notes(&self) -> Result<Vec<Note>, rusqlite::Error> {

        let mut stmt = self.conn.prepare("SELECT id, name, content, created_at FROM Notes")?;

        let note_iter = stmt.query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                name: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;
    
        let mut notes = Vec::new();

        for note in note_iter {
            notes.push(note?);
        }
        Ok(notes)
    }
}