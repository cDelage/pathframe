use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::infrastructure::is_kebab_case;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationPrototype {
    pub application_prototype_metadata: ApplicationPrototypeMetadata,
    pub components: Vec<ComponentMetadata>,
    pub modules: Vec<Module>,
}

/// Metadata of Application prototype.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationPrototypeMetadata {
    pub application_id: String,
    pub application_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub design_system_id: Option<String>,
    #[serde(skip_deserializing)]
    pub application_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ComponentMetadata {
    pub component_id: String,
    pub component_name: SelectorName,
    #[serde(skip_deserializing)]
    pub component_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ModuleMetadata {
    pub module_id: String,
    pub module_name: SelectorName,
    #[serde(skip_deserializing)]
    pub module_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub module_metadata: ModuleMetadata,
    pub pages: Vec<PageMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMetadata {
    pub page_id: String,
    pub page_name: SelectorName,
    #[serde(skip_deserializing)]
    pub page_path: String,
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
pub enum FrameType {
    Layout,
    Page,
    Component,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub template: String,
    pub dataset: String,
    #[serde(skip_deserializing)]
    pub frame_path: String,
    pub frame_type: FrameType,
}

impl Frame {
    pub fn from(frame_type: FrameType) -> Frame {
        let template: String = match frame_type {
            FrameType::Layout => String::from("<body></body>"),
            FrameType::Page => String::from("<main></main>"),
            FrameType::Component => String::from("<div></div>"),
        };

        Frame {
            template,
            dataset: String::new(),
            frame_path: String::new(),
            frame_type,
        }
    }
}
