use serde::{Deserialize, Serialize};

pub mod application_prototype_domain;
pub mod design_system_domain;

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Workspace {
    pub workspace_id: String,
    pub workspace_name: String,
    #[serde(skip_deserializing)]
    pub workspace_path: String
}