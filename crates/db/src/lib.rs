use anyhow::{bail, Result};
use rusqlite::Connection;

pub struct Db {
    conn: Connection,
}

pub struct DbResult {
    pub data: Vec<u8>,
    pub version: String,
}

impl Db {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("pierog.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS cairo_maps (
                hash TEXT PRIMARY KEY,
                data BLOB,
                version TEXT
            )",
            [],
        )?;
        Ok(Db { conn })
    }

    pub fn get(&self, key: &str) -> Result<DbResult> {
        let mut stmt = self
            .conn
            .prepare("SELECT data, version FROM cairo_maps WHERE hash = ?")?;
        let mut rows = stmt.query(rusqlite::params![key])?;
        if let Some(row) = rows.next()? {
            let data: Vec<u8> = row.get(0)?;
            let version: String = row.get(1)?;
            Ok(DbResult { data, version })
        } else {
            bail!("Program not found")
        }
    }

    pub fn set(&self, key: &str, data: &[u8], version: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO cairo_maps (hash, data, version) VALUES (?, ?, ?)",
            rusqlite::params![key, data, version],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Read};

    #[test]
    fn test_db() {
        let db = Db::new().unwrap();
        // get the file from path test.cairo
        let mut file = File::open("test.cairo").unwrap();
        // read file contents into a buffer
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        db.set("0xaa", &buf, "0.13.1").unwrap();

        let db_result = db.get("0xaa").unwrap();
        let retrieved_data = db_result.data;

        // print the file content
        println!("File content: {:?}", retrieved_data);
    }
}
