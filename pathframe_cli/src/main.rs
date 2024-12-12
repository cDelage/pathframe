use args::{ApplicationPrototypeSubCommands, EntityCommands, PathframeArgs};
use clap::Parser;

use colored::*;
use pathframe_lib::workspace::application_prototype::{self, ApplicationPrototypeIndex};
use std::error::Error;

mod args;

fn main() -> Result<(), Box<dyn Error>> {
    let args = PathframeArgs::parse();

    // Match sur les différentes commandes et arguments
    let workspace = &args.workspace.ok_or("Fail to find workspace")?;
    match args.command {
        // Cas de la commande ApplicationPrototype
        EntityCommands::ApplicationPrototype(subcommand) => {
            match subcommand.command {
                // Cas de la sous-commande List
                ApplicationPrototypeSubCommands::List => {
                    let application_prototypes: Vec<ApplicationPrototypeIndex> =
                        application_prototype::find_all_application_prototypes(workspace)?;

                    application_prototypes.iter().for_each(|application| {
                        let app_name = &application.application_name.bright_white();
                        let app_path = &application
                            .application_path
                            .as_ref()
                            .unwrap_or(&String::from("UNKNOWN_PATH"))
                            .truecolor(128, 128, 128);
                        println!("{} - {}", app_name, app_path);
                    });
                }
                ApplicationPrototypeSubCommands::CreateApplication(payload) => {
                    let app_proto_id: String = application_prototype::create_application_prototype(
                        &workspace,
                        &payload.application_name,
                        None,
                    )?;
                    println!("Application successfully created (ID:{})", app_proto_id);
                }
                ApplicationPrototypeSubCommands::FindById(payload) => {
                    let application =
                        application_prototype::find_application_by_id(&workspace, &payload.id)?;

                    println!(
                        "Application : {}",
                        application.application_prototype_index.application_name
                    )
                }
                ApplicationPrototypeSubCommands::CreateModule(payload) => {
                    let application = application_prototype::find_application_by_id(
                        &workspace,
                        &payload.application_id,
                    )?;

                    let module_id = application_prototype::create_module(
                        &application
                            .application_prototype_index
                            .application_path
                            .unwrap(),
                        &payload.module_name,
                    )?;

                    println!("Success to create module (Module ID : {}", module_id);
                }
                ApplicationPrototypeSubCommands::ListModules(payload) => {
                    let application = application_prototype::find_application_by_id(
                        &workspace,
                        &payload.application_id,
                    )?;
                    let modules = application_prototype::find_modules_by_application(
                        &application
                            .application_prototype_index
                            .application_path
                            .unwrap(),
                    )
                    .unwrap_or(vec![]);
                    modules.into_iter().for_each(|module| {
                        let module_name = &module.module_index.module_name.value().bright_white();
                        let module_path = &module
                            .module_index
                            .module_path
                            .as_ref()
                            .unwrap_or(&String::from("UNKNOWN_PATH"))
                            .truecolor(128, 128, 128);
                        println!("{} - {}", module_name, module_path);
                    });
                }
                ApplicationPrototypeSubCommands::CreatePage(payload) => {
                    let application = application_prototype::find_application_by_id(
                        &workspace,
                        &payload.application_id,
                    )?;

                    let module = application_prototype::find_module_by_id(
                        &application
                            .application_prototype_index
                            .application_path
                            .unwrap(),
                        &payload.module_id,
                    )?;

                    let page_id = application_prototype::create_page(
                        &module.module_index.module_path.unwrap(),
                        &payload.page_name,
                    )?;

                    println!("Page created (Page id : {})", page_id);
                }
                ApplicationPrototypeSubCommands::CreateComponent(payload) => {
                    let application = application_prototype::find_application_by_id(
                        &workspace,
                        &payload.application_id,
                    )?;

                    let component_id = application_prototype::create_component(
                        &application
                            .application_prototype_index
                            .application_path
                            .unwrap(),
                        &payload.component_name,
                    )?;

                    println!("Component created (Component id : {})", component_id);
                }
                ApplicationPrototypeSubCommands::ListComponent(payload) => {
                    let application = application_prototype::find_application_by_id(
                        &workspace,
                        &payload.application_id,
                    )?;

                    let components = application_prototype::find_components_by_application(
                        &application
                            .application_prototype_index
                            .application_path
                            .unwrap(),
                    )?;

                    components.iter().for_each(|component| {
                        let component_name = &component.component_name.value().bright_white();
                        let component_path = &component
                            .component_path
                            .as_ref()
                            .unwrap_or(&String::from("UNKNOWN_PATH"))
                            .truecolor(128, 128, 128);
                        println!("{} - {}", component_name, component_path);
                    });
                }
            }
        }
    }

    Ok(())
}
