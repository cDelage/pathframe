use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::infrastructure::is_kebab_case;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationPrototype {
    pub application_prototype_metadata: ApplicationPrototypeMetadata,
    pub components: Vec<FrameMetadata>,
    pub pages: Vec<FrameMetadata>,
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
pub struct FrameName(String);

impl FrameName {
    pub fn parse(name: String) -> Result<FrameName> {
        if is_kebab_case(&name) {
            return Ok(FrameName(name));
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

impl Default for FrameName {
    fn default() -> FrameName {
        FrameName(String::new())
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
    #[serde(skip_deserializing)]
    pub frame_name: FrameName,
    #[serde(skip_deserializing)]
    pub frame_path: String,
    pub frame_type: FrameType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameMetadata {
    pub frame_name: FrameName,
    #[serde(skip_deserializing)]
    pub frame_path: String,
    pub frame_type: FrameType,
}

impl Frame {
    pub fn from(frame_metadata: FrameMetadata) -> Frame {
        let template: String = match frame_metadata.frame_type {
            FrameType::Layout => String::from("<body></body>"),
            FrameType::Page => String::from("<main></main>"),
            FrameType::Component => String::from("<div></div>"),
        };

        Frame {
            template,
            frame_path: frame_metadata.frame_path,
            frame_type: frame_metadata.frame_type,
            frame_name: frame_metadata.frame_name
        }
    }
}
