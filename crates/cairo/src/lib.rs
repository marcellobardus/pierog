use std::{path::PathBuf, process::Stdio};

use tempfile::NamedTempFile;
use tokio::process::Command;
use tracing::debug;

#[cfg(test)]
pub mod tests;

pub async fn cairo_compile(program_path: PathBuf) -> Result<NamedTempFile, std::io::Error> {
    let output = NamedTempFile::new()?;

    let task = Command::new("cairo-compile")
        .arg(program_path.as_path())
        .arg("--output")
        .arg(output.path())
        .stdout(Stdio::null())
        .spawn()?;

    task.wait_with_output().await?;

    debug!("program {:?} is compiling... ", program_path);

    Ok(output)
}

pub async fn compute_hash(compiled_program_path: PathBuf) -> Result<Vec<u8>, std::io::Error> {
    let task = Command::new("cairo-hash-program")
        .arg("--program")
        .arg(compiled_program_path.as_path())
        .arg("--use_poseidon")
        .arg("USE_POSEIDON")
        .stdout(Stdio::piped())
        .spawn()?;

    let output = task.wait_with_output().await?;

    // Remove all whitespaces from the string and skip 0x
    let cleaned: String = String::from_utf8(output.stdout.clone())
        .unwrap()
        .chars()
        .skip(2)
        .filter(|c| !c.is_whitespace())
        .collect();

    debug!("program {:?} is compiling... ", cleaned);

    Ok(hex::decode(cleaned).unwrap())
}