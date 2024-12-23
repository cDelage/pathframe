use anyhow::{Context, Result};
use serde::{Serialize, de::DeserializeOwned};
use serde_yaml;
use serde_json;
use std::fs::DirEntry;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::io::Write;

pub mod workspace_repository;
/// Loads a JSON file from a given directory entry and deserializes it into a generic type `T`.
///
/// # Arguments
///
/// * `dir_entry` - A reference to a `DirEntry` pointing to the directory.
/// * `filename` - The name of the JSON file to load.
///
/// # Returns
///
/// Returns an instance of type `T` if the file is found and deserialized successfully,
/// or an error if the file is not found or deserialization fails.
///
/// # Errors
///
/// This function will return an error in the following cases:
/// - If the file cannot be opened, read, or deserialized.
///
fn load_json<T>(dir_entry: &DirEntry, filename: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let path = dir_entry.path().join(filename);

    let mut file = File::open(&path)
        .context(format!("Failed to open file '{}'", path.display()))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).context(format!(
        "Failed to read file '{}'",
        path.display()
    ))?;

    let data: T = serde_json::from_str(&contents).context("Failed to deserialize JSON")?;

    Ok(data)
}


/// Computes a unique file path in the specified directory.
///
/// This function takes a directory and a filename, replacing spaces in the
/// filename with hyphens. If a file with the generated path already exists,
/// the function appends a numeric suffix (e.g., "file-1", "file-2") to
/// ensure the path is unique.
///
/// # Arguments
///
/// * `directory` - The directory in which the file path should be created.
/// * `filename` - The name of the file, which may contain spaces.
///
/// # Returns
///
/// A `PathBuf` representing the unique file path.
pub fn compute_file_path(directory: &PathBuf, filename: &str) -> PathBuf {
    let mut file_path = directory.join(filename.replace(" ", "-"));

    if file_path.exists() {
        let mut i = 1;
        while Path::new(&format!("{} ({})", file_path.display(), i)).exists() {
            i += 1;
        }
        file_path = PathBuf::from(format!("{} ({})", file_path.display(), i));
    }

    file_path
}

fn save_to_yaml_file<P, T>(path: P, data: &T) -> Result<()>
where
    P: AsRef<Path>,
    T: Serialize,
{
    let yaml_data = serde_yaml::to_string(data)
        .expect("Échec de la sérialisation en YAML");

    let mut file = File::create(path)?;
    file.write_all(yaml_data.as_bytes())?;
    Ok(())
}

fn concat_path(left: &str, right: &str) -> PathBuf {
    Path::join(
        Path::new(left),
        Path::new(right),
    )
}


pub fn is_kebab_case(input: &str) -> bool {
    let kebab_case_pattern = regex::Regex::new(r"^[a-z0-9]+([-_][a-z0-9]+)*$").unwrap();
    kebab_case_pattern.is_match(input)
}

/// Loads a YAML file from a given directory iterator and deserializes it into a generic type `T`.
///
/// # Arguments
///
/// * `read_dir_result` - A `Result<ReadDir, std::io::Error>` containing the directory iterator or an error.
/// * `filename` - The name of the YAML file to load.
///
/// # Returns
///
/// Returns an instance of type `T` if the file is found and deserialized successfully,
/// or an error if the file is not found or deserialization fails.
///
/// # Errors
///
/// This function will return an error in the following cases:
/// - If the directory cannot be read.
/// - If the file with the specified name is not found in the directory.
/// - If the file cannot be opened, read, or deserialized.
fn load_yaml<T>(dir_entry: &DirEntry, filename: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let path = dir_entry.path().join(filename);

    let mut file = File::open(&path)
        .context(format!("Failed to open file '{}'", path.display()))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).context(format!(
        "Failed to read file '{}'",
        path.display()
    ))?;

    let data: T = serde_yaml::from_str(&contents).context("Failed to deserialize YAML")?;

    Ok(data)
}

/// Loads a YAML file from a given directory iterator and deserializes it into a generic type `T`.
///
/// # Arguments
///
/// * `read_dir_result` - A `Result<ReadDir, std::io::Error>` containing the directory iterator or an error.
/// * `filename` - The name of the YAML file to load.
///
/// # Returns
///
/// Returns an instance of type `T` if the file is found and deserialized successfully,
/// or an error if the file is not found or deserialization fails.
///
/// # Errors
///
/// This function will return an error in the following cases:
/// - If the directory cannot be read.
/// - If the file with the specified name is not found in the directory.
/// - If the file cannot be opened, read, or deserialized.
fn load_yaml_from_pathbuf<T>(pathbuf: &PathBuf) -> Result<T>
where
    T: DeserializeOwned,
{
    let mut file = File::open(&pathbuf)
        .context(format!("Failed to open file '{}'", pathbuf.display()))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).context(format!(
        "Failed to read file '{}'",
        pathbuf.display()
    ))?;

    let data: T = serde_yaml::from_str(&contents).context("Failed to deserialize YAML")?;

    Ok(data)
}