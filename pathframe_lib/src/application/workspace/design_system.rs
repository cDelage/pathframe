use std::{
    fs,
    path::{Path, PathBuf},
};
use anyhow::{anyhow, Result};
use crate::domain::workspace_domain::design_system_domain::DesignSystem;
use super::load_json;

const DESIGN_SYSTEMS_PATH: &str = "design_systems";
const DESIGN_SYSTEM_INDEX_PATH: &str = "design_system.json";

pub fn find_all_design_systems(workspace_path: &str) -> Result<Vec<DesignSystem>> {
    let folder_path: PathBuf = Path::new(workspace_path).join(DESIGN_SYSTEMS_PATH);
    let read_dir = fs::read_dir(&folder_path)?;

    Ok(read_dir
        .into_iter()
        .filter_map(|dir_entry| {
            let dir = dir_entry.ok()?;

            if !dir.path().is_dir() || !dir.path().join(DESIGN_SYSTEM_INDEX_PATH).is_file() {
                return None;
            }
            let design_system = load_json::<DesignSystem>(&dir, DESIGN_SYSTEM_INDEX_PATH).ok()?;
            Some(design_system)
        })
        .collect::<Vec<DesignSystem>>())
}

pub fn find_design_system_by_id(
    workspace_path: &str,
    design_system_id: &str,
) -> Result<DesignSystem> {
    let design_systems = find_all_design_systems(workspace_path)?;

    design_systems
        .into_iter()
        .find(|ds| ds.design_system_id == design_system_id)
        .ok_or_else(|| anyhow!("Design system with ID {} not found", design_system_id))
}
