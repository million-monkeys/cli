use clap::ArgEnum;
use clap::{Parser, Subcommand};
use strum_macros::Display;

#[macro_use]
extern crate maplit;

pub mod components;
pub mod events;
pub mod module;
pub mod project;
pub mod utils;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage projects
    Project {
        /// Project name
        name: String,

        #[clap(subcommand)]
        command: ProjectCommands,
    },
    /// Run development mode
    Dev {},
    /// Manage modules
    Module {
        /// Module name
        name: String,

        /// Selects a project
        #[clap(short, long, value_name = "PROJECT")]
        project: Option<String>,

        #[clap(subcommand)]
        command: ModuleCommands,
    },
    /// Generate Code
    Generate {
        #[clap(subcommand)]
        command: GenerateCommands,
    },
}

#[derive(Subcommand)]
enum ProjectCommands {
    /// Create a new project
    Create,
    /// Build a project
    Build {
        // Directory to build to (default: projects/<project>/build)
        #[clap(short, long, value_name = "BUILDDIR")]
        builddir: Option<String>,
    },
    /// Run development mode on project
    Dev,
    /// Run a project in the engine
    Run {
        /// Choose engine build
        #[clap(short, long, arg_enum)]
        build: Option<project::RunBuild>,
    },
    /// Create a release package for a project
    Release,
}

#[derive(Subcommand)]
enum ModuleCommands {
    /// Create a new module
    New,
    /// Build a module
    Build,
}

#[derive(Subcommand)]
enum GenerateCommands {
    /// Generate code from TOML definition
    Components {
        #[clap(arg_enum)]
        build: GeneratorTypes,
        /// Source TOML file
        source: String,
        /// Destination directory
        destination: String,
    },
}

#[derive(Clone, ArgEnum)]
enum GeneratorTypes {
    Lua,
    Hpp,
    Cpp,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Project { name, command } => match command {
            ProjectCommands::Create => project::create(name),
            ProjectCommands::Build { builddir } => project::build(name, builddir),
            ProjectCommands::Dev => project::dev(name),
            ProjectCommands::Run { build } => {
                project::run(name, &build.unwrap_or(project::RunBuild::Release))
            }
            ProjectCommands::Release => project::release(name),
        },
        Commands::Dev {} => println!("NOT IMPLEMENTED: dev"),
        Commands::Module {
            name,
            project,
            command,
        } => match command {
            ModuleCommands::New => module::new(name, project),
            ModuleCommands::Build => module::build(name, project),
        },
        Commands::Generate { command } => match command {
            GenerateCommands::Components {
                build,
                source,
                destination,
            } => match build {
                GeneratorTypes::Lua => components::generate(
                    source,
                    components::GeneratorType::LuaDefinitions,
                    destination,
                ),
                GeneratorTypes::Cpp => components::generate(
                    source,
                    components::GeneratorType::CppDefinitions,
                    destination,
                ),
                GeneratorTypes::Hpp => {
                    components::generate(source, components::GeneratorType::HeaderFile, destination)
                }
            },
        },
    }

    // Continued program logic goes here...
}
