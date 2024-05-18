use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::{Child, Command, Stdio};

use serde::Deserialize;
use tempfile::{NamedTempFile, TempDir};

use axum::extract::Multipart;
use zip::ZipArchive;

#[derive(Deserialize, Debug)]
pub enum Compiler {
    Cairo,
}

#[derive(Debug)]
pub struct CompilationRunner {
    compiler: Compiler,
}

impl CompilationRunner {
    pub fn new(compiler: Compiler) -> Self {
        Self { compiler }
    }

    pub async fn prepare_files(mut multipart: Multipart) -> Result<bool, String> {
        let temp_dir = TempDir::new();
        if temp_dir.is_err() {
            println!("Failed to create temporary directory");
            return Err("Failed to create temporary directory".to_string());
        }
        let temp_dir = temp_dir.unwrap();

        // print temp dir
        println!("Temp dir: {:?}", temp_dir.path());

        // Create the ZIP file from the multipart request
        let field = multipart.next_field().await.unwrap();
        if field.is_none() {
            println!("No file found in the request");
            return Err("No file found in the request".to_string());
        }
        let content = field.unwrap().bytes().await.unwrap();
        let file = NamedTempFile::new();
        if file.is_err() {
            println!("Failed to create temporary file");
            return Err("Failed to create temporary file".to_string());
        }
        let mut zip_file = file.unwrap();
        zip_file.write_all(&content).unwrap();

        // Create the archive
        let archive = ZipArchive::new(zip_file);
        if archive.is_err() {
            println!("Failed to open zip archive");
            return Err("Failed to open zip archive".to_string());
        }
        let mut archive = archive.unwrap();

        // Iterate over each file in the zip archive
        for i in 0..archive.len() {
            let file = archive.by_index(i);

            if file.is_err() {
                println!("Failed to read file from zip archive");
                return Err("Failed to read file from zip archive".to_string());
            }
            let mut file = file.unwrap();

            println!("Extracted file: {}", file.name());

            let outpath = temp_dir.path().join(file.name());
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
            }
        }

        // TODO: Bart -> return files you need to compile in the format you prefer and pass them over to the `compile` method.

        // TODO: Pia part -> store the zip file in database.

        Ok(true)
    }

    pub async fn compile(&self) -> Result<String, String> {
        println!("Compiling with {:?} compiler", self.compiler);

        let child: Result<Child, std::io::Error>;
        match self.compiler {
            Compiler::Cairo => {
                // TODO: Bart part -> invoke cairo_compile

                let args = ["-l"]; // TODO: define *actual* compilation arguments.
                child = Command::new("ls").args(args).stdout(Stdio::piped()).spawn();
            }
        }
        if child.is_err() {
            println!("Failed to spawn child process");

            return Err(format!("Failed to compile: {}", child.unwrap_err()));
        }

        match child.unwrap().wait_with_output() {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                println!("Compilation output: {}", output_str);

                Ok(output_str.to_string())
            }
            Err(err) => Err(format!("Failed on waiting with output: {}", err)),
        }
    }
}

#[cfg(test)]
mod tests {}
