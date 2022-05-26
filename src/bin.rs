use bevy_cg_lib::*;
use clap::Parser;
use std::str::FromStr;

/// Select what bevy project to generate
#[derive(Parser)]
struct Cli {
    #[clap(arg_enum, default_value_t = Template::Default)]
    template: Template,

    #[clap(multiple_occurrences(true), arg_enum)]
    commands: Vec<Commands>,
}

#[derive(clap::ArgEnum, Clone)]
enum Template
{
    Default,
    Plugin,
    Basic2D,
    Basic3D,
}
//Templates
//Default : empty main/game with wasm support
//Plugin : basic plugin
//2D : Very basic 2D game
//3D : Very basic 3D game

#[derive(clap::ArgEnum, PartialEq, Clone)]
enum Commands
{
    Default,
    Code,
    Clean,
    Release,
}
//Commands
//cmd default - Does the whole process except for the clean, release and code cmd
//cmd code
//cmd clean
//cmd release

impl FromStr for Commands {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "default" => Commands::Default,
            "code" => Commands::Code,
            "clean" => Commands::Clean,
            "release" => Commands::Release,
            _ => Commands::Default
        })
    }

    type Err = std::string::ParseError;
}

fn main() {
    let args = Cli::parse();
    
    let bevy_model = match args.template {
        Template::Default => create_default_template(),
        Template::Plugin => create_plugin_template(),
        Template::Basic2D => todo!(),
        Template::Basic3D => todo!(),
    };

    let scope = bevy_model.generate();

    println!("{}", scope.to_string());

    let serialized = serde_json::to_string(&bevy_model).unwrap();
    println!("serialized = {}", serialized);

    let _ = write_to_file(bevy_model.clone());

    if args.commands.contains(&Commands::Clean){
        cmd_clean(bevy_model.clone());
    }
    
    if args.commands.contains(&Commands::Default){
        cmd_default(bevy_model.clone(), false);
    }
    
    if args.commands.contains(&Commands::Release){
        cmd_release(bevy_model.clone());
    }

    if args.commands.contains(&Commands::Code){
        cmd_code(bevy_model);
    }
}