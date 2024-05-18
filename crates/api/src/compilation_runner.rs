use cairo::{cairo_compile, compute_hash};
use db::Db;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use tempfile::{NamedTempFile, TempDir};
use zip::ZipArchive;

#[derive(Deserialize, Debug)]
pub enum Compiler {
    Cairo,
}

#[derive(Debug)]
pub struct CompilationRunner {
    workspace_root_path: PathBuf,
    target_compilation_path: PathBuf,
    compiler: Compiler,
}

impl CompilationRunner {
    pub fn new(
        compiler: Compiler,
        workspace_root_path: PathBuf,
        target_compilation_path: PathBuf,
    ) -> Self {
        Self {
            compiler,
            workspace_root_path,
            target_compilation_path,
        }
    }

    pub async fn prepare_files(zip_data: Vec<u8>) -> Result<bool, String> {
        let temp_dir = TempDir::new();
        if temp_dir.is_err() {
            println!("Failed to create temporary directory");
            return Err("Failed to create temporary directory".to_string());
        }
        let temp_dir = temp_dir.unwrap();

        let file = NamedTempFile::new();
        if file.is_err() {
            println!("Failed to create temporary file");
            return Err("Failed to create temporary file".to_string());
        }
        let mut zip_file = file.unwrap();
        zip_file.write_all(&zip_data).unwrap();

        // Create the archive
        let mut received_file = NamedTempFile::new().unwrap();
        received_file.write_all(&zip_data).unwrap();
        let archive = ZipArchive::new(zip_file);
        if let Err(e) = archive {
            println!("Failed to open zip archive: {}", e);
            return Err(format!("Failed to open zip archive: {}", e));
        }
        let mut archive = archive.unwrap();

        let temp_dir_path = temp_dir.path();
        // Iterate over each file in the zip archive
        for i in 0..archive.len() {
            let file = archive.by_index(i);

            if file.is_err() {
                println!("Failed to read file from zip archive");
                return Err("Failed to read file from zip archive".to_string());
            }
            let mut file = file.unwrap();

            println!("Extracted file: {}", file.name());

            let outpath = temp_dir_path.join(file.name());
            println!("Outpath: {:?}", outpath);

            // Create the directory structure for the file
            if let Some(p) = outpath.parent() {
                fs::create_dir_all(p).unwrap()
            }

            // If the entry is a file, extract it to the temporary directory
            if file.is_file() {
                let mut outfile = fs::File::create(&outpath).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
                // Read content of the file
                let mut buf = String::new();
                let mut file = File::open(&outpath).unwrap();
                file.read_to_string(&mut buf).unwrap();
                println!("File content: {}", buf);
            }
        }

        // TODO: Bart -> return files you need to compile in the format you prefer and pass them over to the `compile` method.

        // TODO: Pia part -> store the zip file in database.
        let db = Db::new().unwrap();
        db.set("0xaa", &received_file, "0.13.1").unwrap();
        Ok(true)
    }

    pub async fn compile(&self) -> Result<Vec<u8>, String> {
        println!("Compiling with {:?} compiler", self.compiler);

        let hash = match self.compiler {
            Compiler::Cairo => {
                let compiled_cairo = cairo_compile(
                    self.workspace_root_path.to_owned(),
                    self.target_compilation_path.to_owned(),
                )
                .await
                .unwrap();
                compute_hash(compiled_cairo.path().to_path_buf())
                    .await
                    .unwrap()
            }
        };

        Ok(hash)
    }
}

#[cfg(test)]
mod tests {}
