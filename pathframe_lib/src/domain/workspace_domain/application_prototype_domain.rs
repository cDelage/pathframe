use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::application::workspace::is_kebab_case;

/// Metadata of Application prototype.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationPrototypeIndex {
    pub application_id: String,
    pub application_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub design_system_id: Option<String>,
    #[serde(skip_deserializing)]
    pub application_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Frame {
    pub template: String,
    pub dataset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ComponentIndex {
    pub component_id: String,
    pub component_name: SelectorName,
    #[serde(skip_deserializing)]
    pub component_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ModuleIndex {
    pub module_id: String,
    pub module_name: SelectorName,
    #[serde(skip_deserializing)]
    pub module_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub module_index: ModuleIndex,
    pub pages: Vec<PageIndex>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageIndex {
    pub page_id: String,
    pub page_name: SelectorName,
    #[serde(skip_deserializing)]
    pub page_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectorName(String);

impl SelectorName {
    pub fn parse(name: String) -> Result<SelectorName> {
        if is_kebab_case(&name) {
            return Ok(SelectorName(name));
        }
        Err(anyhow!(
            "Fail to parse selector name in snake case: {}",
            name
        ))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationPrototype {
    pub application_prototype_index: ApplicationPrototypeIndex,
    pub components: Vec<ComponentIndex>,
    pub modules: Vec<Module>,
}
