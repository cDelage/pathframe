use anyhow::Result;

use crate::{domain::workspace_domain::Workspace, infrastructure::workspace_repository, utils::generate_uuid};

pub mod application_prototype_service;
pub mod design_system_service;


pub fn create_workspace(workspace_path: &String) -> Result<Workspace> {
    let workspace: Workspace = Workspace {
        workspace_id: generate_uuid(),
        workspace_path: workspace_path.clone()
    };

    println!("workspace : {:?}", workspace);

    workspace_repository::create_workspace(&workspace)?;
    
    Ok(workspace)
}

pub fn find_workspace_by_path(workspace_path: &String) -> Result<Workspace> {
    workspace_repository::find_workspace_by_path(&workspace_path)
}
