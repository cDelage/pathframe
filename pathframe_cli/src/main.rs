use args::{ApplicationPrototypeSubCommands, EntityCommands, PathframeArgs};
use clap::Parser;

use pathframe_lib::workspace::application_prototype::find_all_application_prototypes;
use std::error::Error;

mod args;

fn main() -> Result<(), Box<dyn Error>>{
    let args = PathframeArgs::parse();

    // Match sur les différentes commandes et arguments
    let workspace = &args.workspace.ok_or("Fail to find workspace")?;
    match args.command {
        // Cas de la commande ApplicationPrototype
        EntityCommands::ApplicationPrototype(subcommand) => {
            match subcommand.command {
                // Cas de la sous-commande List
                ApplicationPrototypeSubCommands::List => {
                    let application_prototypes = find_all_application_prototypes(workspace)?;
                    
                    application_prototypes.iter().for_each(|application| {
                        println!("{}", application.application_name);
                        println!("{}", application.description);
                        println!("--------------------------------")
                    });
                    
                }
            }
        }
    }

    Ok(())
}
