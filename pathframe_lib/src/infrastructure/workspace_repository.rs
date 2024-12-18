use std::{
    fs,
    path::PathBuf,
};

use crate::domain::workspace_domain::Workspace;
use anyhow::Result;

use super::{concat_path, load_yaml_from_pathbuf, save_to_yaml_file};

pub mod application_prototype_repository;
pub mod design_system_repository;

pub const ASSETS_PATH: &str = "assets";
pub const DESIGN_SYSTEMS_PATH: &str = "design_systems";
pub const APPLICATION_PROTOTYPES_PATH: &str = "application_prototypes";
pub const WORKSPACE_METADATA_PATH: &str = "workspace_metadata.yaml";

pub fn create_workspace(workspace: &Workspace) -> Result<()> {
    let applications_path = concat_path(&workspace.workspace_path, APPLICATION_PROTOTYPES_PATH);
    let design_systems_path = concat_path(&workspace.workspace_path, DESIGN_SYSTEMS_PATH);
    let assets_path = concat_path(&workspace.workspace_path, ASSETS_PATH);
    fs::create_dir(applications_path)?;
    fs::create_dir(design_systems_path)?;
    fs::create_dir(assets_path)?;
    let workspace_metadata_path: PathBuf =
        concat_path(&workspace.workspace_path, WORKSPACE_METADATA_PATH);
    save_to_yaml_file(workspace_metadata_path, &workspace)?;
    Ok(())
}

pub fn find_workspace_by_path(workspace_path: &String) -> Result<Workspace> {
    let applications_path = concat_path(&workspace_path, APPLICATION_PROTOTYPES_PATH);
    let design_systems_path = concat_path(&workspace_path, DESIGN_SYSTEMS_PATH);
    let assets_path = concat_path(&workspace_path, ASSETS_PATH);
    fs::read_dir(applications_path)?;
    fs::read_dir(design_systems_path)?;
    fs::read_dir(assets_path)?;
    let workspace_pathbuf: PathBuf = PathBuf::from(workspace_path).join(WORKSPACE_METADATA_PATH);
    let mut workspace: Workspace = load_yaml_from_pathbuf::<Workspace>(&workspace_pathbuf)?;
    workspace.workspace_path = String::from(workspace_path);
    Ok(workspace)
}
