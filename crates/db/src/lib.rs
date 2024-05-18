use anyhow::Result;
use rusqlite::Connection;
use std::{
    fs::File,
    io::{Read, Write},
};

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("pierog.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS cairo_maps (
                hash TEXT PRIMARY KEY,
                data TEXT
            )",
            [],
        )?;
        Ok(Db { conn })
    }

    pub fn get(&self, key: &str) -> Result<Option<File>> {
        let mut stmt = self
            .conn
            .prepare("SELECT data FROM cairo_maps WHERE hash = ?")?;
        let mut rows = stmt.query([key])?;
        if let Some(row) = rows.next()? {
            let data: String = row.get(0)?;
            let mut file = tempfile::NamedTempFile::new()?;
            file.write_all(data.as_bytes())?;
            Ok(Some(file.reopen()?))
        } else {
            Ok(None)
        }
    }

    pub fn set(&self, key: &str, mut value: File) -> Result<()> {
        let mut buf = String::new();
        value.read_to_string(&mut buf)?;
        self.conn.execute(
            "INSERT OR REPLACE INTO cairo_maps (hash, data) VALUES (?, ?)",
            &[key, &buf],
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
        let file = File::open("test.cairo").unwrap();

        db.set("0xaa", file).unwrap();
        let mut retrieved_file = db.get("0xaa").unwrap().unwrap();

        let mut buf = String::new();
        retrieved_file.read_to_string(&mut buf).unwrap();

        // turn this into file object
        print!("{}", buf);
    }
}
