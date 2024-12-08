use anyhow::Result;
use std::fs::{self, ReadDir};
use std::path::Path;

use crate::workspace::{
    is_file_exist, is_folder, load_yaml, ApplicationPrototypeIndex, APPLICATION_PROTOTYPES_PATH,
    APPLICATION_PROTOTYPE_INDEX_PATH,
};

pub fn find_all_application_prototypes(
    workspace_path: &str,
) -> Result<Vec<ApplicationPrototypeIndex>> {
    println!("Find all application prototypes");
    //let mut directories: Vec<> = Vec::new();
    let folder_path = Path::join(
        Path::new(workspace_path),
        Path::new(APPLICATION_PROTOTYPES_PATH),
    );
    let read_dir: ReadDir = fs::read_dir(&folder_path)?;

    Ok(read_dir
        .into_iter()
        .filter(is_folder)
        .filter(|dir_entry| is_file_exist(dir_entry, &APPLICATION_PROTOTYPE_INDEX_PATH))
        .filter_map(|dir_entry| {
            load_yaml::<ApplicationPrototypeIndex>(dir_entry, &APPLICATION_PROTOTYPE_INDEX_PATH)
                .ok()
        })
        .collect::<Vec<ApplicationPrototypeIndex>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all() {
        let workspace_path = "tests\\assets\\fake_workspace";
        let test_workspace = find_all_application_prototypes(workspace_path);
        match test_workspace {
            Ok(workspace) => {
                workspace
                    .iter()
                    .for_each(|result| println!("Application : {:?}", result));
                let applications: Vec<String> = workspace
                    .into_iter()
                    .map(|index| index.application_name)
                    .collect();
                assert!(applications.contains(&String::from("PathFrame")));
            }
            Err(_) => {
                panic!("Fail to load workspace")
            }
        }
    }
}
