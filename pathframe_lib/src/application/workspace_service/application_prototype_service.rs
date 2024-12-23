use crate::domain::workspace_domain::application_prototype_domain::{
    ApplicationPrototype, ApplicationPrototypeMetadata, FrameMetadata, FrameName, FrameType,
};
use crate::domain::workspace_domain::Workspace;
use crate::infrastructure::workspace_repository::application_prototype_repository;
use crate::utils::generate_uuid;

use anyhow::{anyhow, Ok, Result};

pub fn find_all_application_prototypes(
    workspace: &Workspace,
) -> Result<Vec<ApplicationPrototypeMetadata>> {
    application_prototype_repository::find_all_application_prototypes(workspace)
}

pub fn create_application_prototype(
    workspace: &Workspace,
    application_name: &str,
    description: Option<String>,
) -> Result<ApplicationPrototypeMetadata> {
    let id = generate_uuid();
    let mut application_prototype = ApplicationPrototypeMetadata {
        application_id: id.clone(),
        application_name: String::from(application_name),
        description,
        design_system_id: None,
        application_path: String::new(),
    };
    application_prototype_repository::create_application_repository(
        workspace,
        &mut application_prototype,
    )?;
    Ok(application_prototype)
}

pub fn find_application_by_id(
    workspace: &Workspace,
    application_id: &str,
) -> Result<ApplicationPrototype> {
    let applications: Vec<ApplicationPrototypeMetadata> =
        application_prototype_repository::find_all_application_prototypes(workspace)?;

    let application_prototype_metadata: ApplicationPrototypeMetadata = applications
        .into_iter()
        .find(|application| application.application_id == application_id)
        .ok_or_else(|| anyhow!("Application with ID {} not found", application_id))?;

    let components: Vec<FrameMetadata> = application_prototype_repository::find_frames_metadata(
        &application_prototype_metadata,
        FrameType::Component,
    )?;

    let pages: Vec<FrameMetadata> = application_prototype_repository::find_frames_metadata(
        &application_prototype_metadata,
        FrameType::Page,
    )?;

    Ok(ApplicationPrototype {
        application_prototype_metadata: application_prototype_metadata.clone(),
        components,
        pages,
    })
}

pub fn find_components_by_application(
    application_prototype_metadata: &ApplicationPrototypeMetadata,
) -> Result<Vec<FrameMetadata>> {
    application_prototype_repository::find_frames_metadata(
        application_prototype_metadata,
        FrameType::Component,
    )
}

pub fn find_pages_by_application(
    application_prototype_metadata: &ApplicationPrototypeMetadata,
) -> Result<Vec<FrameMetadata>> {
    application_prototype_repository::find_frames_metadata(
        application_prototype_metadata,
        FrameType::Page,
    )
}

pub fn create_component(
    application_prototype_metadata: &ApplicationPrototypeMetadata,
    component_name: &str,
) -> Result<FrameMetadata> {
    let component_name_parsed: FrameName = FrameName::parse(String::from(component_name))?;

    let mut component_metadata: FrameMetadata = FrameMetadata {
        frame_name: component_name_parsed,
        frame_path: String::new(),
        frame_type: FrameType::Component,
    };

    application_prototype_repository::create_frame(
        &application_prototype_metadata,
        &mut component_metadata,
    )?;

    return Ok(component_metadata);
}

pub fn create_page(
    application_prototype_metadata: &ApplicationPrototypeMetadata,
    page_name: &str,
) -> Result<FrameMetadata> {
    let page_name_parsed = FrameName::parse(String::from(page_name))?;

    let mut page_metadata = FrameMetadata {
        frame_name: page_name_parsed,
        frame_path: String::new(),
        frame_type: FrameType::Page,
    };

    application_prototype_repository::create_frame(
        &application_prototype_metadata,
        &mut page_metadata,
    )?;

    Ok(page_metadata)
}
