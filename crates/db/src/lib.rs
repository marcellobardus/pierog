use anyhow::{bail, Result};
use rusqlite::Connection;
use std::{
    fs::File,
    io::{Read, Write},
};
use tempfile::NamedTempFile;

pub struct Db {
    conn: Connection,
}

pub struct DbResult {
    pub data: File,
    pub version: String,
}

impl Db {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("pierog.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS cairo_maps (
                hash TEXT PRIMARY KEY,
                data TEXT,
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
        let mut rows = stmt.query([key])?;
        if let Some(row) = rows.next()? {
            let data: String = row.get(0)?;
            let version: String = row.get(1)?;
            let mut file = tempfile::NamedTempFile::new()?;
            file.write_all(data.as_bytes())?;
            Ok(DbResult {
                data: file.reopen()?,
                version,
            })
        } else {
            bail!("Program not found")
        }
    }

    pub fn set(&self, key: &str, data: NamedTempFile, version: &str) -> Result<()> {
        // turn NamedTempFile into File
        let mut file_data = data.reopen()?;
        let mut buf = String::new();
        file_data.read_to_string(&mut buf)?;
        self.conn.execute(
            "INSERT OR REPLACE INTO cairo_maps (hash, data, version) VALUES (?, ?, ?)",
            [key, &buf, version],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn test_db() {
        let db = Db::new().unwrap();
        // get the file from path test.cairo
        let mut file = File::open("test.cairo").unwrap();
        // turn file to NamedTempFile
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        temp_file.write_all(&buf).unwrap();
        db.set("0xaa", temp_file, "0.13.1").unwrap();
        let db_result = db.get("0xaa").unwrap();
        let mut retrieved_file = db_result.data;
        let mut buf = String::new();
        retrieved_file.read_to_string(&mut buf).unwrap();
        // turn this into file object
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        temp_file.write_all(buf.as_bytes()).unwrap();
        // print the file content
        println!("{}", buf);
    }
}
