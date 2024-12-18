use serde::{Deserialize, Serialize};

pub mod application_prototype_domain;
pub mod design_system_domain;
pub mod design_system_to_stylesheet_domain;

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Workspace {
    pub workspace_id: String,
    #[serde(skip_deserializing)]
    pub workspace_path: String
}