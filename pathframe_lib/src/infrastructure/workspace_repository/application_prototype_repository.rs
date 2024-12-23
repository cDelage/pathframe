use crate::{
    domain::workspace_domain::{
        application_prototype_domain::{
            ApplicationPrototypeMetadata, Frame, FrameMetadata, FrameName,
            FrameType,
        },
        Workspace,
    },
    infrastructure::{compute_file_path, concat_path, load_yaml, save_to_yaml_file},
};
use anyhow::Result;
use std::{ffi::OsStr, fs::DirEntry, io::Write};
use std::{
    fs::{self, File, ReadDir},
    path::PathBuf,
};

use super::APPLICATION_PROTOTYPES_PATH;

const APPLICATION_PROTOTYPE_METADATA_PATH: &str = "application_metadata.yaml";
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
            if !dir.path().is_dir()
                || !dir
                    .path()
                    .join(APPLICATION_PROTOTYPE_METADATA_PATH)
                    .is_file()
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

pub fn save_application_prototype_metadata(
    application_prototype: &ApplicationPrototypeMetadata,
) -> Result<()> {
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
    let folder_pathbuf: PathBuf =
        concat_path(&workspace.workspace_path, APPLICATION_PROTOTYPES_PATH);
    let application_path: PathBuf = compute_file_path(
        &folder_pathbuf,
        &application_prototype_metadata.application_name,
    );
    application_prototype_metadata.application_path =
        application_path.to_string_lossy().into_owned();
    fs::create_dir(&application_path)?;

    let pages_path: PathBuf = application_path.join(PAGES_PATH);
    fs::create_dir(&pages_path)?;

    let images_path: PathBuf = application_path.join(IMAGES_PATH);
    fs::create_dir(&images_path)?;

    let components_path: PathBuf = application_path.join(COMPONENTS_PATH);
    fs::create_dir(&components_path)?;

    let layout_path: PathBuf = application_path.join(LAYOUT_PATH);
    fs::create_dir(&layout_path)?;

    let application_metadata_path: PathBuf = concat_path(
        &application_prototype_metadata.application_path,
        APPLICATION_PROTOTYPE_METADATA_PATH,
    );

    save_to_yaml_file(application_metadata_path, application_prototype_metadata)?;

    Ok(())
}

pub fn find_frames_metadata(
    application_prototype_metadata: &ApplicationPrototypeMetadata,
    frame_type: FrameType,
) -> Result<Vec<FrameMetadata>> {
    let frame_uri = get_frame_uri(&frame_type);
    let frames_path: PathBuf =
        concat_path(&application_prototype_metadata.application_path, frame_uri);

    let read_dir: ReadDir = fs::read_dir(&frames_path)?;

    Ok(read_dir
        .into_iter()
        .filter_map(|dir_entry| {
            let page_entry: DirEntry = dir_entry.ok()?;
            let page_entry_path: PathBuf = page_entry.path();
            let extension: &OsStr = page_entry_path.extension()?;
            if extension != "html" {
                return None;
            }

            let frame_ostr: &OsStr = page_entry_path.file_stem()?;
            let frame_name_str = frame_ostr.to_str()?;
            let frame_name: FrameName = FrameName::parse(String::from(frame_name_str)).ok()?;
            Some(FrameMetadata {
                frame_name,
                frame_path: page_entry.path().to_string_lossy().into_owned(),
                frame_type: frame_type.to_owned().clone(),
            })
        })
        .collect::<Vec<FrameMetadata>>())
}

pub fn create_frame(
    application_prototype_metadata: &ApplicationPrototypeMetadata,
    frame_metadata: &mut FrameMetadata,
) -> Result<()> {
    let uri: &str = get_frame_uri(&frame_metadata.frame_type);
    let components_path: PathBuf =
        concat_path(&application_prototype_metadata.application_path, uri);
    let component_file: String = format!("{}.html", &frame_metadata.frame_name.value());
    let frame_pathbuf: PathBuf = components_path.join(component_file);
    frame_metadata.frame_path = frame_pathbuf.to_string_lossy().into_owned();
    let frame: Frame = Frame::from(frame_metadata.clone());
    let mut file: File = File::create(frame_pathbuf)?;
    write!(file, "{}", frame.template)?;
    return Ok(());
}

fn get_frame_uri(frame_type: &FrameType) -> &str {
    match frame_type {
        FrameType::Component => COMPONENTS_PATH,
        FrameType::Page => PAGES_PATH,
        FrameType::Layout => "",
    }
}
