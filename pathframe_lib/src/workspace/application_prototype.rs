use super::{
    compute_file_path, concat_path, generate_uuid, is_kebab_case, load_yaml, save_to_yaml_file,
};
use anyhow::{anyhow, Ok, Result};
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::fs::{self, ReadDir};
use std::io::Write;
use std::path::PathBuf;

const APPLICATION_PROTOTYPES_PATH: &str = "application_prototypes";
const MODULES_PATH: &str = "modules";
const PAGES_PATH: &str = "pages";
const IMAGES_PATH: &str = "images";
const COMPONENTS_PATH: &str = "components";
const LAYOUT_PATH: &str = "layout";
const APPLICATION_PROTOTYPE_INDEX_PATH: &str = "application_index.yaml";
const MODULE_INDEX_PATH: &str = "module_index.yaml";
const PAGE_INDEX_PATH: &str = "page_index.yaml";
const COMPONENT_INDEX_PATH: &str = "component-index.yaml";
const FRAME_TEMPLATE_PATH: &str = "template.html";
const FRAME_DATASET_PATH: &str = "dataset.yaml";
const PRESET_LAYOUT_TEMPLATE: &str = "<body></body>";
const PRESET_PAGE_TEMPLATE: &str = "<main></main>";
const PRESET_COMPONENT_TEMPLATE: &str = "<div></div>";

/// Metadata of Application prototype.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationPrototypeIndex {
    pub application_id: String,
    pub application_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub design_system_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ModuleIndex {
    pub module_id: String,
    pub module_name: SelectorName,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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

pub fn find_all_application_prototypes(
    workspace_path: &str,
) -> Result<Vec<ApplicationPrototypeIndex>> {
    let folder_path = concat_path(workspace_path, APPLICATION_PROTOTYPES_PATH);
    let read_dir: ReadDir = fs::read_dir(&folder_path)?;

    Ok(read_dir
        .into_iter()
        .filter_map(|dir_entry| {
            let dir = dir_entry.ok()?;
            if !dir.path().is_dir() || !dir.path().join(APPLICATION_PROTOTYPE_INDEX_PATH).is_file()
            {
                return None;
            }
            load_yaml::<ApplicationPrototypeIndex>(&dir, &APPLICATION_PROTOTYPE_INDEX_PATH)
                .ok()
                .map(|mut app_proto_index| {
                    app_proto_index.application_path =
                        Some(dir.path().to_string_lossy().into_owned());
                    app_proto_index
                })
        })
        .collect::<Vec<ApplicationPrototypeIndex>>())
}

pub fn create_application_prototype(
    workspace_path: &str,
    application_name: &str,
    description: Option<String>,
) -> Result<String> {
    let folder_pathbuf = concat_path(workspace_path, APPLICATION_PROTOTYPES_PATH);

    let application_path = compute_file_path(&folder_pathbuf, application_name);

    fs::create_dir(&application_path)?;

    let id = generate_uuid();

    let application_prototype = ApplicationPrototypeIndex {
        application_id: id.clone(),
        application_name: String::from(application_name),
        description,
        design_system_id: None,
        application_path: None,
    };

    let application_index_path = &application_path.join(APPLICATION_PROTOTYPE_INDEX_PATH);
    save_to_yaml_file(application_index_path, &application_prototype)?;

    let modules_path = &application_path.join(MODULES_PATH);
    fs::create_dir(modules_path)?;

    let images_path = &application_path.join(IMAGES_PATH);
    fs::create_dir(images_path)?;

    let components_path = &application_path.join(COMPONENTS_PATH);
    fs::create_dir(components_path)?;

    let layout_path = &application_path.join(LAYOUT_PATH);
    fs::create_dir(layout_path)?;

    create_frame(layout_path, String::from(PRESET_LAYOUT_TEMPLATE))?;

    Ok(id)
}

fn create_frame(frame_path: &PathBuf, template_content: String) -> Result<String> {
    let template_path = frame_path.join(FRAME_TEMPLATE_PATH);
    let mut file = File::create(template_path)?;
    write!(file, "{}", template_content)?;
    let dataset_path = frame_path.join(FRAME_DATASET_PATH);
    File::create(dataset_path)?;
    Ok(String::new())
}

pub fn find_application_by_id(
    workspace_path: &str,
    application_id: &str,
) -> Result<ApplicationPrototype> {
    let applications = find_all_application_prototypes(workspace_path)?;

    let application_prototype_index = applications
        .into_iter()
        .find(|application| application.application_id == application_id)
        .ok_or_else(|| anyhow!("Application with ID {} not found", application_id))?;

    if let Some(path) = &application_prototype_index.application_path {
        return Ok(ApplicationPrototype {
            application_prototype_index: application_prototype_index.clone(),
            components: find_components_by_application(path)?,
            modules: find_modules_by_application(path)?,
        });
    }

    Err(anyhow!("Error"))
}

pub fn find_modules_by_application(application_path: &str) -> Result<Vec<Module>> {
    let modules_path = concat_path(application_path, MODULES_PATH);
    let read_dir: ReadDir = fs::read_dir(&modules_path)?;

    Ok(read_dir
        .filter_map(|dir_entry| {
            let dir = dir_entry.ok()?;
            if !dir.path().is_dir() || !dir.path().join(MODULE_INDEX_PATH).is_file() {
                return None;
            }
            let mut module_index = load_yaml::<ModuleIndex>(&dir, &MODULE_INDEX_PATH).ok()?;
            let path = dir.path().to_string_lossy().into_owned();
            module_index.module_path = Some(path.clone());
            let pages = find_pages_by_module(&path).unwrap_or(vec![]);
            Some(Module {
                module_index,
                pages,
            })
        })
        .collect())
}

pub fn find_module_by_id(application_path: &str, module_id: &str) -> Result<Module> {
    let modules: Vec<Module> = find_modules_by_application(application_path)?;

    modules
        .into_iter()
        .find(|module| module.module_index.module_id == module_id)
        .ok_or_else(|| anyhow!("Module with ID {} not found", module_id))
}

pub fn find_pages_by_module(module_path: &str) -> Result<Vec<PageIndex>> {
    let pages_path = concat_path(module_path, PAGES_PATH);
    let read_dir: ReadDir = fs::read_dir(&pages_path)?;

    Ok(read_dir
        .into_iter()
        .filter_map(|dir_entry| {
            let dir = dir_entry.ok()?;
            if !dir.path().is_dir() || !dir.path().join(PAGE_INDEX_PATH).is_file() {
                return None;
            }
            load_yaml::<PageIndex>(&dir, &PAGES_PATH)
                .ok()
                .map(|mut page_index| {
                    page_index.page_path = Some(dir.path().to_string_lossy().into_owned());
                    page_index
                })
        })
        .collect::<Vec<PageIndex>>())
}

pub fn find_components_by_application(application_path: &str) -> Result<Vec<ComponentIndex>> {
    let components_path = concat_path(application_path, COMPONENTS_PATH);

    let read_dir = fs::read_dir(components_path)?;

    Ok(read_dir
        .into_iter()
        .filter_map(|component_dir| {
            let dir = component_dir.ok()?;

            if !&dir.path().is_dir() || !&dir.path().join(COMPONENT_INDEX_PATH).is_file() {
                return None;
            }

            load_yaml::<ComponentIndex>(&dir, COMPONENT_INDEX_PATH)
                .ok()
                .map(|mut index| {
                    index.component_path = Some(dir.path().to_string_lossy().into_owned());
                    index
                })
        })
        .collect::<Vec<ComponentIndex>>())
}

pub fn create_component(application_path: &str, component_name: &str) -> Result<String> {
    let component_name_parser: SelectorName = SelectorName::parse(String::from(component_name))?;

    let components_dir: PathBuf = PathBuf::from(application_path).join(COMPONENTS_PATH);

    let component_path: PathBuf =
        compute_file_path(&components_dir, &component_name_parser.value());

    fs::create_dir(&component_path)?;

    let component_index_path: &PathBuf = &component_path.join(COMPONENT_INDEX_PATH);

    let component_id: String = generate_uuid();

    let component_index = ComponentIndex {
        component_id: component_id.clone(),
        component_name: component_name_parser,
        component_path: None,
    };

    save_to_yaml_file(component_index_path, &component_index)?;

    create_frame(&component_path, String::from(PRESET_COMPONENT_TEMPLATE))?;

    return Ok(component_id);
}

pub fn create_module(application_path: &str, module_name: &str) -> Result<String> {
    let module_name_parsed: SelectorName = SelectorName::parse(String::from(module_name))?;

    let module_dir: PathBuf = PathBuf::from(application_path).join(MODULES_PATH);
    let module_path: PathBuf = compute_file_path(&module_dir, &module_name_parsed.value());

    fs::create_dir(&module_path)?;

    let module_id: String = generate_uuid();

    let module_pages_path: &PathBuf = &module_path.join(PAGES_PATH);
    fs::create_dir(&module_pages_path)?;

    let module_index: ModuleIndex = ModuleIndex {
        module_id: module_id.clone(),
        module_name: module_name_parsed,
        module_path: None,
    };

    let module_index_path: &PathBuf = &module_path.join(MODULE_INDEX_PATH);
    save_to_yaml_file(module_index_path, &module_index)?;

    return Ok(module_id);
}

pub fn create_page(module_path: &str, page_name: &str) -> Result<String> {
    let page_name_parsed = SelectorName::parse(String::from(page_name))?;

    let pages_dir = PathBuf::from(module_path).join(PAGES_PATH);
    let pages_path = compute_file_path(&pages_dir, &page_name_parsed.value());

    fs::create_dir(&pages_path)?;

    let page_id: String = generate_uuid();

    let page_index = PageIndex {
        page_id: page_id.clone(),
        page_name: page_name_parsed,
        page_path: None,
    };

    let page_index_path: PathBuf = pages_path.join(APPLICATION_PROTOTYPE_INDEX_PATH);

    save_to_yaml_file(page_index_path, &page_index)?;

    create_frame(&pages_path, String::from(PRESET_PAGE_TEMPLATE))?;

    Ok(page_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all() {
        let workspace_path = "tests\\assets\\fake_workspace";
        let test_workspace = find_all_application_prototypes(workspace_path);
        test_workspace.unwrap().len();
    }
}
