use crate::{domain::workspace_domain::{
    application_prototype_domain::{
        ApplicationPrototypeMetadata, ComponentMetadata, Frame, ModuleMetadata, PageMetadata,
    },
    Workspace,
}, infrastructure::{compute_file_path, concat_path, load_yaml, save_to_yaml_file}};
use anyhow::Result;
use std::{fs::DirEntry, io::Write};
use std::{
    fs::{self, File, ReadDir},
    path::PathBuf,
};

const APPLICATION_PROTOTYPE_METADATA_PATH: &str = "application_metadata.yaml";
const MODULE_METADATA_PATH: &str = "module_metadata.yaml";
const PAGE_METADATA_PATH: &str = "page_metadata.yaml";
const COMPONENT_METADATA_PATH: &str = "component_metadata.yaml";
const FRAME_TEMPLATE_PATH: &str = "template.html";
const FRAME_DATASET_PATH: &str = "dataset.yaml";
const APPLICATION_PROTOTYPES_PATH: &str = "application_prototypes";
const MODULES_PATH: &str = "modules";
const PAGES_PATH: &str = "pages";
const IMAGES_PATH: &str = "images";
const COMPONENTS_PATH: &str = "components";
const LAYOUT_PATH: &str = "layout";

pub fn find_all_application_prototypes(
    workspace: &Workspace,
) -> Result<Vec<ApplicationPrototypeMetadata>> {
    let folder_path: PathBuf = concat_path(&workspace.workspace_path, APPLICATION_PROTOTYPES_PATH);
    let read_dir: ReadDir = fs::read_dir(&folder_path)?;

    Ok(read_dir
        .filter_map(|dir_entry| {
            let dir = dir_entry.ok()?;
            if !dir.path().is_dir() || !dir.path().join(APPLICATION_PROTOTYPE_METADATA_PATH).is_file()
            {
                return None;
            }
            load_yaml::<ApplicationPrototypeMetadata>(&dir, APPLICATION_PROTOTYPE_METADATA_PATH)
                .ok()
                .map(|mut app_proto_metadata| {
                    app_proto_metadata.application_path = dir.path().to_string_lossy().into_owned();
                    app_proto_metadata
                })
        })
        .collect())
}

pub fn save_application_prototype_metadata(application_prototype: &ApplicationPrototypeMetadata) -> Result<()> {
    save_to_yaml_file(
        &application_prototype.application_path,
        application_prototype,
    )?;
    Ok(())
}

///Create folder structure for application
/// With modules, images, components, layout folders
pub fn create_application_repository(
    workspace: &Workspace,
    application_prototype_metadata: &mut ApplicationPrototypeMetadata,
) -> Result<()> {
    let folder_pathbuf = concat_path(&workspace.workspace_path, APPLICATION_PROTOTYPES_PATH);
    let application_path =
        compute_file_path(&folder_pathbuf, &application_prototype_metadata.application_name);
    application_prototype_metadata.application_path = application_path.to_string_lossy().into_owned();
    let modules_path = application_path.join(MODULES_PATH);
    fs::create_dir(modules_path)?;

    let images_path = application_path.join(IMAGES_PATH);
    fs::create_dir(images_path)?;

    let components_path = application_path.join(COMPONENTS_PATH);
    fs::create_dir(components_path)?;

    let layout_path = application_path.join(LAYOUT_PATH);
    fs::create_dir(layout_path)?;

    save_to_yaml_file(
        &application_prototype_metadata.application_path,
        application_prototype_metadata,
    )?;

    Ok(())
}

pub fn create_frame_layout(application: &ApplicationPrototypeMetadata, frame: &mut Frame) -> Result<()> {
    let layout_path: PathBuf = concat_path(&application.application_path, LAYOUT_PATH);
    frame.frame_path = layout_path.to_string_lossy().into_owned();
    create_frame(frame)
}

pub fn create_frame_component(component_metadata: &ComponentMetadata, frame: &mut Frame) -> Result<()> {
    frame.frame_path = component_metadata.component_path.clone();
    create_frame(frame)
}

pub fn create_page_frame(page_metadata: &PageMetadata, frame: &mut Frame) -> Result<()> {
    frame.frame_path = page_metadata.page_path.clone();
    create_frame(frame)
}

fn create_frame(frame: &Frame) -> Result<()> {
    let frame_path: PathBuf  = PathBuf::from(&frame.frame_path);
    let template_path = frame_path.join(FRAME_TEMPLATE_PATH);
    let mut file = File::create(template_path)?;
    write!(file, "{}", frame.template)?;
    let dataset_path = frame_path.join(FRAME_DATASET_PATH);
    File::create(dataset_path)?;
    Ok(())
}

pub fn find_all_modules_metadata(
    application_prototype_metadata: &ApplicationPrototypeMetadata,
) -> Result<Vec<ModuleMetadata>> {
    let modules_path = concat_path(&application_prototype_metadata.application_path, MODULES_PATH);
    let read_dir: ReadDir = fs::read_dir(&modules_path)?;

    Ok(read_dir
        .filter_map(|dir_entry| {
            let dir = dir_entry.ok()?;
            if !dir.path().is_dir() || !dir.path().join(MODULE_METADATA_PATH).is_file() {
                return None;
            }
            let mut module_metadata = load_yaml::<ModuleMetadata>(&dir, &MODULE_METADATA_PATH).ok()?;
            let path = dir.path().to_string_lossy().into_owned();
            module_metadata.module_path = path.clone();
            Some(module_metadata)
        })
        .collect::<Vec<ModuleMetadata>>())
}

pub fn find_all_page_metadata(module_metadata: &ModuleMetadata) -> Result<Vec<PageMetadata>> {
    let pages_path = concat_path(&module_metadata.module_path, PAGES_PATH);
    let read_dir: ReadDir = fs::read_dir(&pages_path)?;

    Ok(read_dir
        .into_iter()
        .filter_map(|dir_entry| {
            let dir: DirEntry = dir_entry.ok()?;
            if !dir.path().is_dir() || !dir.path().join(PAGE_METADATA_PATH).is_file() {
                return None;
            }
            load_yaml::<PageMetadata>(&dir, &PAGES_PATH)
                .ok()
                .map(|mut page_metadata| {
                    page_metadata.page_path = dir.path().to_string_lossy().into_owned();
                    page_metadata
                })
        })
        .collect::<Vec<PageMetadata>>())
}

pub fn find_all_component_metadata(
    application_prototype_metadata: &ApplicationPrototypeMetadata,
) -> Result<Vec<ComponentMetadata>> {
    let components_path = concat_path(
        &application_prototype_metadata.application_path,
        COMPONENTS_PATH,
    );

    let read_dir = fs::read_dir(components_path)?;

    Ok(read_dir
        .into_iter()
        .filter_map(|component_dir| {
            let dir = component_dir.ok()?;

            if !&dir.path().is_dir() || !&dir.path().join(COMPONENT_METADATA_PATH).is_file() {
                return None;
            }

            load_yaml::<ComponentMetadata>(&dir, COMPONENT_METADATA_PATH)
                .ok()
                .map(|mut component_metadata| {
                    component_metadata.component_path = dir.path().to_string_lossy().into_owned();
                    component_metadata
                })
        })
        .collect::<Vec<ComponentMetadata>>())
}


pub fn create_component_metadata(application_prototype_metadata: &ApplicationPrototypeMetadata, component_metadata: &mut ComponentMetadata) -> Result<()> {
    let components_dir: PathBuf = PathBuf::from(&application_prototype_metadata.application_path).join(COMPONENTS_PATH);
    let component_path: PathBuf =
        compute_file_path(&components_dir, &component_metadata.component_name.value());

    fs::create_dir(&component_path)?;

    let component_metadata_path: &PathBuf = &component_path.join(COMPONENT_METADATA_PATH);
    
    save_to_yaml_file(component_metadata_path, component_metadata)?;

    component_metadata.component_path = component_metadata_path.to_string_lossy().into_owned();

    return Ok(());
}

pub fn create_module_metadata(application_prototype_metadata: &ApplicationPrototypeMetadata, module_metadata: &mut ModuleMetadata) -> Result<()> {
    let module_dir: PathBuf = PathBuf::from(&application_prototype_metadata.application_path).join(MODULES_PATH);
    let module_path: PathBuf = compute_file_path(&module_dir, &module_metadata.module_name.value());
    fs::create_dir(&module_path)?;
    let module_pages_path: &PathBuf = &module_path.join(PAGES_PATH);
    fs::create_dir(&module_pages_path)?;
    let module_metadata_path: &PathBuf = &module_path.join(MODULE_METADATA_PATH);
    save_to_yaml_file(module_metadata_path, &module_metadata)?;
    module_metadata.module_path = module_path.to_string_lossy().into_owned();
    Ok(())
}

pub fn create_page_metadata(module_metadata: &ModuleMetadata, page_metadata: &mut PageMetadata) -> Result<()> {
    let pages_dir = PathBuf::from(&module_metadata.module_path).join(PAGES_PATH);
    let page_path = compute_file_path(&pages_dir, &page_metadata.page_name.value());
    fs::create_dir(&page_path)?;

    let page_metadata_path: PathBuf = page_path.join(APPLICATION_PROTOTYPE_METADATA_PATH);
    save_to_yaml_file(page_metadata_path, &page_metadata)?;

    page_metadata.page_path = page_path.to_string_lossy().into_owned();
    Ok(())
}
