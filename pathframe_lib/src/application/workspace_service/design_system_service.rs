use anyhow::{anyhow, Result};
use crate::domain::workspace_domain::{design_system_domain::DesignSystem, Workspace};
use crate::infrastructure::workspace_repository::design_system_repository;

pub fn find_all_design_systems(workspace: &Workspace) -> Result<Vec<DesignSystem>> {
    design_system_repository::find_all_design_systems(&workspace)
}   

pub fn find_design_system_by_id(
    workspace: &Workspace,
    design_system_id: &str,
) -> Result<DesignSystem> {
    let design_systems: Vec<DesignSystem> = design_system_repository::find_all_design_systems(&workspace)?;

    design_systems
        .into_iter()
        .find(|ds| ds.design_system_id == design_system_id)
        .ok_or_else(|| anyhow!("Design system with ID {} not found", design_system_id))
}
