use anyhow::{anyhow, Result};
use args::{ApplicationPrototypeSubCommands, EntityCommands, PathframeArgs};
use clap::Parser;

use colored::*;
use pathframe_lib::{
    application::workspace_service::{
        self, application_prototype_service,
        design_system_service::{
            design_system_to_stylesheet, find_all_design_systems, find_design_system_by_id,
        },
    },
    domain::workspace_domain::{
        application_prototype_domain::{
            ApplicationPrototype, ApplicationPrototypeMetadata, ComponentMetadata, Module,
            ModuleMetadata, PageMetadata,
        },
        design_system_domain::DesignSystem,
        Workspace,
    },
};

mod args;

fn main() -> Result<()> {
    let args = PathframeArgs::parse();

    // Match sur les différentes commandes et arguments
    let workspace_path = &args
        .workspace
        .ok_or(anyhow!("Fail to find workspace path"))?;
    let workspace_option: Option<Workspace> = match &args.command {
        EntityCommands::CreateWorkspace => None,
        _ => Some(workspace_service::find_workspace_by_path(&workspace_path)?),
    };

    match args.command {
        // Cas de la commande ApplicationPrototype
        EntityCommands::CreateWorkspace => {
            println!("Create workspace");
            let workspace = workspace_service::create_workspace(workspace_path)?;

            println!(
                "Succeed to init workspace (workspace_id : {})",
                workspace.workspace_id
            );
        }
        EntityCommands::ApplicationPrototype(subcommand) => {
            println!("workspace: {:?}", workspace_option);
            let workspace: Workspace = workspace_option.ok_or(anyhow!("fail to find workspace"))?;
            match subcommand.command {
                // Cas de la sous-commande List
                ApplicationPrototypeSubCommands::List => {
                    let application_prototypes: Vec<ApplicationPrototypeMetadata> =
                        application_prototype_service::find_all_application_prototypes(&workspace)?;

                    application_prototypes.iter().for_each(|application| {
                        let app_name: &ColoredString = &application.application_name.bright_white();
                        let app_path: &ColoredString =
                            &application.application_path.truecolor(128, 128, 128);
                        println!("{} - {}", app_name, app_path);
                    });
                }
                ApplicationPrototypeSubCommands::CreateApplication(payload) => {
                    let app_proto: ApplicationPrototypeMetadata =
                        application_prototype_service::create_application_prototype(
                            &workspace,
                            &payload.application_name,
                            None,
                        )?;
                    println!(
                        "Application successfully created (ID:{})",
                        app_proto.application_id
                    );
                }
                ApplicationPrototypeSubCommands::FindById(payload) => {
                    let application: ApplicationPrototype =
                        application_prototype_service::find_application_by_id(
                            &workspace,
                            &payload.id,
                        )?;

                    println!(
                        "Application : {}",
                        application.application_prototype_metadata.application_name
                    )
                }
                ApplicationPrototypeSubCommands::CreateModule(payload) => {
                    let application = application_prototype_service::find_application_by_id(
                        &workspace,
                        &payload.application_id,
                    )?;

                    let module: ModuleMetadata = application_prototype_service::create_module(
                        &application.application_prototype_metadata,
                        &payload.module_name,
                    )?;

                    println!("Success to create module (Module ID : {}", module.module_id);
                }
                ApplicationPrototypeSubCommands::CreatePage(payload) => {
                    let application = application_prototype_service::find_application_by_id(
                        &workspace,
                        &payload.application_id,
                    )?;

                    let module: Module = application_prototype_service::find_module_by_id(
                        &application.application_prototype_metadata,
                        &payload.module_id,
                    )?;

                    let page_metadata: PageMetadata = application_prototype_service::create_page(
                        &module.module_metadata,
                        &payload.page_name,
                    )?;

                    println!("Page created (Page id : {})", page_metadata.page_id);
                }
                ApplicationPrototypeSubCommands::CreateComponent(payload) => {
                    let application: ApplicationPrototype =
                        application_prototype_service::find_application_by_id(
                            &workspace,
                            &payload.application_id,
                        )?;

                    let component: ComponentMetadata =
                        application_prototype_service::create_component(
                            &application.application_prototype_metadata,
                            &payload.component_name,
                        )?;

                    println!(
                        "Component created (Component id : {})",
                        component.component_id
                    );
                }
                ApplicationPrototypeSubCommands::ListComponent(payload) => {
                    let application = application_prototype_service::find_application_by_id(
                        &workspace,
                        &payload.application_id,
                    )?;

                    let components: Vec<ComponentMetadata> =
                        application_prototype_service::find_components_by_application(
                            &application.application_prototype_metadata,
                        )?;

                    components.iter().for_each(|component| {
                        let component_name = &component.component_name.value().bright_white();
                        let component_path = &component.component_path.truecolor(128, 128, 128);
                        println!("{} - {}", component_name, component_path);
                    });
                }
            }
        }
        EntityCommands::DesignSystem(subcommand) => {
            let workspace: Workspace = workspace_option.ok_or(anyhow!("fail to find workspace"))?;
            match subcommand.command {
                args::DesignSystemSubCommands::List => {
                    let all_design_systems: Vec<DesignSystem> =
                        find_all_design_systems(&workspace)?;
                    all_design_systems
                        .iter()
                        .for_each(|design_system: &DesignSystem| {
                            println!("{}", design_system.design_system_name)
                        });
                }
                args::DesignSystemSubCommands::FindById(payload) => {
                    let design_system: DesignSystem =
                        find_design_system_by_id(&workspace, &payload.design_system_id)?;
                    println!("{}", design_system.design_system_name);
                }
                args::DesignSystemSubCommands::ToStylesheet(payload) => {
                    let design_system: String =
                        design_system_to_stylesheet(&workspace, &payload.design_system_id)?;
                    println!("{}", design_system);
                }
            }
        }
    }

    Ok(())
}
