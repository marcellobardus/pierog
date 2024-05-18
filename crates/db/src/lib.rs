use anyhow::{bail, Result};
use rusqlite::Connection;
use std::io::Read;
use tempfile::NamedTempFile;

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

    pub fn set(&self, key: &str, data: &NamedTempFile, version: &str) -> Result<()> {
        let mut file = data.reopen()?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        self.conn.execute(
            "INSERT OR REPLACE INTO cairo_maps (hash, data, version) VALUES (?, ?, ?)",
            rusqlite::params![key, buf, version],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::File,
        io::{Read, Write},
    };

    #[test]
    fn test_db() {
        let db = Db::new().unwrap();
        // get the file from path test.cairo
        let mut file = File::open("test.cairo").unwrap();
        // read file contents into a buffer
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        // create a NamedTempFile from the buffer
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(&buf).unwrap();
        db.set("0xaa", &temp_file, "0.13.1").unwrap();

        let db_result = db.get("0xaa").unwrap();
        let retrieved_data = db_result.data;

        // print the file content
        println!("File content: {:?}", retrieved_data);
    }
}
