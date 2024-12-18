use crate::domain::workspace_domain::application_prototype_domain::{
    ApplicationPrototype, ApplicationPrototypeMetadata, ComponentMetadata, Frame, FrameType, Module,
    ModuleMetadata, PageMetadata, SelectorName,
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
    let mut frame: Frame = Frame::from(FrameType::Layout);
    application_prototype_repository::create_frame_layout(&application_prototype, &mut frame)?;
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

    let modules_metadata: Vec<ModuleMetadata> =
        application_prototype_repository::find_all_modules_metadata(&application_prototype_metadata)?;

    let modules = modules_metadata
        .into_iter()
        .filter_map(|module_metadata| {
            let pages: Vec<PageMetadata> =
                application_prototype_repository::find_all_page_metadata(&module_metadata).ok()?;
            Some(Module {
                module_metadata,
                pages,
            })
        })
        .collect::<Vec<Module>>();

    let components: Vec<ComponentMetadata> =
        application_prototype_repository::find_all_component_metadata(&application_prototype_metadata)?;

    Ok(ApplicationPrototype {
        application_prototype_metadata: application_prototype_metadata.clone(),
        components,
        modules,
    })
}

pub fn find_module_by_id(application_prototype_metadata: &ApplicationPrototypeMetadata, module_id: &str) -> Result<Module> {
    let modules_metadata: Vec<ModuleMetadata> = application_prototype_repository::find_all_modules_metadata(application_prototype_metadata)?;

    let module_metadata: ModuleMetadata = modules_metadata
        .into_iter()
        .find(|module| module.module_id == module_id)
        .ok_or_else(|| anyhow!("Module with ID {} not found", module_id))?;

    let pages: Vec<PageMetadata> = application_prototype_repository::find_all_page_metadata(&module_metadata)?;
    
    Ok(Module {
        module_metadata,
        pages
    })
}

pub fn find_components_by_application(application_prototype_metadata: &ApplicationPrototypeMetadata) -> Result<Vec<ComponentMetadata>> {
    application_prototype_repository::find_all_component_metadata(application_prototype_metadata)
}

pub fn create_component(application_prototype_metadata: &ApplicationPrototypeMetadata, component_name: &str) -> Result<ComponentMetadata> {
    let component_name_parsed: SelectorName = SelectorName::parse(String::from(component_name))?;

    let mut component_metadata: ComponentMetadata = ComponentMetadata {
        component_id: generate_uuid(),
        component_name: component_name_parsed,
        component_path: String::new()    
    };

    application_prototype_repository::create_component_metadata(&application_prototype_metadata, &mut component_metadata)?;

    return Ok(component_metadata);
}

pub fn create_module(application_prototype_metadata: &ApplicationPrototypeMetadata, module_name: &str) -> Result<ModuleMetadata> {
    let module_name_parsed: SelectorName = SelectorName::parse(String::from(module_name))?;

    let mut module_metadata: ModuleMetadata = ModuleMetadata {
        module_id: generate_uuid(),
        module_name: module_name_parsed,
        module_path: String::new(),
    };

    application_prototype_repository::create_module_metadata(&application_prototype_metadata, &mut module_metadata)?;

    return Ok(module_metadata);
}

pub fn create_page(module_metadata: &ModuleMetadata, page_name: &str) -> Result<PageMetadata> {
    let page_name_parsed = SelectorName::parse(String::from(page_name))?;
    
    let mut page_metadata = PageMetadata {
        page_id: generate_uuid(),
        page_name: page_name_parsed,
        page_path: String::new(),
    };

    application_prototype_repository::create_page_metadata(&module_metadata, &mut page_metadata)?;
    
    let mut frame: Frame = Frame::from(FrameType::Layout);

    application_prototype_repository::create_page_frame(&page_metadata, &mut frame)?;

    Ok(page_metadata)
}