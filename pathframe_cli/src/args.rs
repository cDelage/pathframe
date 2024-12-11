use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "pframe", about = "Pathframe CLI tool")]
pub struct PathframeArgs {
    #[command(subcommand)]
    pub command: EntityCommands,
    #[arg(short = 'w', long = "workspace")]
    pub workspace: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum EntityCommands {
    ///Manage the application prototypes (list, read...)
    #[command(aliases=["app","app-proto"])]
    ApplicationPrototype(ApplicationPrototypeCommands),
}

#[derive(Debug, Args)]
pub struct ApplicationPrototypeCommands {
    #[clap(subcommand)]
    pub command: ApplicationPrototypeSubCommands,
}

#[derive(Debug, Subcommand)]
pub enum ApplicationPrototypeSubCommands {
    ///Find all application prototypes into workspace
    List,
    #[command(aliases=["create"])]
    CreateApplication(CreateApplicationArgs),
    FindById(FindApplicationByIdArgs),
    CreateModule(CreateModuleArgs),
    ListModules(ListModulesArgs)
}

#[derive(Debug, Parser)]
pub struct CreateApplicationArgs {
    #[arg(aliases=["name"])]
    pub application_name: String
}

#[derive(Debug, Parser)]
pub struct FindApplicationByIdArgs {
    pub id: String
}

#[derive(Debug, Parser)]
pub struct CreateModuleArgs {
    #[arg(aliases=["app-id"])]
    pub application_id: String,
    
    #[arg(aliases=["mod-name","name"])]
    pub module_name: String,
}

#[derive(Debug, Parser)]
pub struct ListModulesArgs {
    #[arg(aliases=["app-id"])]
    pub application_id: String,
}

