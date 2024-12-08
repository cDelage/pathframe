use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};
use serde_yaml;
use std::fs::DirEntry;
use std::fs::File;
use std::io::Read;

pub mod application_prototype;

const APPLICATION_PROTOTYPES_PATH: &str = "application_prototypes";
const APPLICATION_PROTOTYPE_INDEX_PATH: &str = "application_index.yaml";

/// Metadata of Application prototype.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationPrototypeIndex {
    pub application_id: String,
    pub application_name: String,
    pub description: String,
    pub design_system_id: String,
}

/// Vérifie si un `DirEntry` est un dossier.
///
/// # Arguments
///
/// * `dir_entry` - Une référence à un `DirEntry` représentant une entrée de dossier.
///
/// # Returns
///
/// * `true` si l'entrée est un dossier, sinon `false`.
fn is_folder(dir_entry: &Result<DirEntry, std::io::Error>) -> bool {
    match dir_entry {
        Ok(result) => result.path().is_dir(),
        Err(_) => false,
    }
}

/// Checks if a file with a given name exists in the path corresponding to a directory entry.
///
/// # Arguments
///
/// * `dir_entry` - A reference to a result containing a directory entry (`DirEntry`) or an error (`std::io::Error`).
///     - If it is an error (`Err`), the function returns `false`.
///     - If it is a valid entry (`Ok`), the function checks if a file with the specified name exists in the directory.
/// * `filename` - A string representing the name of the file to search for.
///
/// # Returns
///
/// * `true` if a file with the specified name exists in the directory corresponding to the provided entry.
/// * `false` if the entry is an error or if the file does not exist.
///
fn is_file_exist(dir_entry: &Result<DirEntry, std::io::Error>, filename: &str) -> bool {
    match dir_entry {
        Ok(result) => result.path().join(filename).is_file(),
        Err(_) => false,
    }
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
fn load_yaml<T>(dir_entry: Result<DirEntry, std::io::Error>, filename: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let dir = dir_entry?;

    let mut file = File::open(dir.path().join(filename))
        .context(format!("Failed to open file '{}'", dir.path().display()))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).context(format!(
        "Failed to read file '{}'",
        dir.path().join(filename).display()
    ))?;

    let data: T = serde_yaml::from_str(&contents).context("Failed to deserialize YAML")?;

    Ok(data)
}
