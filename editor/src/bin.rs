use bevy_codegen::{
    commands::{cmd_clean, cmd_code, cmd_default, cmd_release},
    templates::{default_game::create_default_template, default_plugin::create_plugin_template},
    write_to_file,
};
use clap::Parser;
use std::str::FromStr;

use crate::ui::start_editor;

pub mod ui;

/// Select what bevy project to generate
#[derive(Parser)]
struct Cli {
    #[clap(value_enum, default_value_t = Template::Default)]
    template: Template,

    #[clap(value_enum)]
    commands: Vec<Commands>,
}

#[derive(clap::ValueEnum, Clone)]
enum Template {
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

#[derive(clap::ValueEnum, PartialEq, Clone)]
enum Commands {
    Default,
    Code,
    Clean,
    Release,
    Editor,
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
            _ => Commands::Default,
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

    if args.commands.contains(&Commands::Clean) {
        cmd_clean(bevy_model.clone());
    }

    if args.commands.contains(&Commands::Default) {
        cmd_default(bevy_model.clone(), false);
    }

    if args.commands.contains(&Commands::Release) {
        cmd_release(bevy_model.clone());
    }

    if args.commands.contains(&Commands::Code) {
        cmd_code(bevy_model);
    }

    if args.commands.contains(&Commands::Editor) {
        println!("Starting editor");
        start_editor();
    }
}
