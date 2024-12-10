use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};
use serde_yaml;
use uuid::Uuid;
use std::fs::DirEntry;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::io::Write;

pub mod application_prototype;

const APPLICATION_PROTOTYPES_PATH: &str = "application_prototypes";
const MODULES_PATH: &str = "modules";
const APPLICATION_PROTOTYPE_INDEX_PATH: &str = "application_index.yaml";

/// Metadata of Application prototype.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationPrototypeIndex {
    pub application_id: String,
    pub application_name: String,
    pub description: Option<String>,
    pub design_system_id: Option<String>,
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

fn generate_uuid() -> String{
    Uuid::new_v4().to_string()
}

fn save_to_yaml_file<P: AsRef<Path>>(path: P, data: &ApplicationPrototypeIndex) -> std::io::Result<()> {
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