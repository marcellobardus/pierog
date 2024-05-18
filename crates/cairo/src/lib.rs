use std::{path::PathBuf, process::Stdio};

use tempfile::NamedTempFile;
use tokio::process::Command;
use tracing::debug;

#[cfg(test)]
pub mod tests;

pub async fn cairo_compile(
    workspace_root_path: PathBuf,
    target_compilation_path: PathBuf,
) -> Result<NamedTempFile, std::io::Error> {
    let output = NamedTempFile::new()?;

    let task = Command::new("cairo-compile")
        .arg(target_compilation_path.as_path())
        .arg("--output")
        .arg(output.path())
        .arg("--cairo_path")
        .arg(workspace_root_path.as_path())
        .stdout(Stdio::null())
        .spawn()?;

    task.wait_with_output().await?;

    debug!("program {:?} is compiling... ", target_compilation_path);

    Ok(output)
}

pub async fn compute_hash(compiled_program_path: PathBuf) -> Result<String, std::io::Error> {
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

    println!("program {:?} is compiling... ", cleaned);

    Ok(cleaned)
}
