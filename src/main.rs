extern crate codegen;
use codegen::{Function, Scope};
use serde::{Serialize, Deserialize};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
};

#[derive(Serialize, Deserialize)]
struct BevyModel {
    #[serde(skip)]
    scope: Scope,
}

trait BevyCodegen {
    fn create_app(&mut self, content: &str) -> &mut Function;

    fn create_query(&mut self, content: &str) -> &mut Function;
}
impl BevyCodegen for Scope {
    fn create_app(&mut self, content: &str) -> &mut Function {
        self.raw("#[bevy_main]")
            .new_fn("main")
            .line(format!("App::new(){}.run();", content))
    }

    fn create_query(&mut self, content: &str) -> &mut Function {
        self.new_fn(format!("{}_system", content).as_str())
        .line(format!("println!(\"{}\");", content))
    }
}

fn main() {
    let mut bevy_model = BevyModel {
        scope: Scope::new(),
    };

    bevy_model.scope.import("bevy::prelude", "*");

    let hw = "hello_world";
    let hws = format!(".add_system({}_system)", hw);
    bevy_model
        .scope
        .create_app(&hws);

    bevy_model
        .scope
        .create_query(&hw);

    println!("{}", bevy_model.scope.to_string());

    let serialized = serde_json::to_string(&bevy_model).unwrap();
    println!("serialized = {}", serialized);

    let _ = write_to_file(bevy_model);

    build_and_run();
}

fn build_and_run() {
    println!("fmt");
    let _fmt = Command::new("cargo")
        .arg("fmt")
        .arg("--all")
        .current_dir("bevy")
        .status() //output()
        .expect("failed to execute cargo fmt");

    println!("fix");
    let _fix = Command::new("cargo")
        .arg("clippy")
        .arg("--fix")
        .arg("--allow-no-vcs")
        .current_dir("bevy")
        .status() //output()
        .expect("failed to execute cargo fix");

    println!("clippy");
    let _clippy = Command::new("cargo")
        .arg("clippy")
        .arg("--")
        .arg("-D")
        .arg("warnings")
        .current_dir("bevy")
        .status() //output()
        .expect("failed to execute cargo clippy");

    //println!("{:?}", clippy);

    println!("run");
    let _run = Command::new("cargo")
        .arg("run")
        .current_dir("bevy")
        .status() //output()
        .expect("failed to execute cargo run");
}

fn write_to_file(model: BevyModel) -> std::io::Result<()> {
    const BEVY_FOLDER: &str = "bevy";
    const SRC_FOLDER: &str = "src";
    if Path::new(BEVY_FOLDER).exists() {
        fs::remove_dir_all(BEVY_FOLDER.to_owned() + "/" + &SRC_FOLDER.to_owned())?;
        let _rf = fs::remove_file(BEVY_FOLDER.to_owned() + "/" + "Cargo.toml");
    } else {
        fs::create_dir(BEVY_FOLDER)?;
    }
    fs::create_dir(BEVY_FOLDER.to_owned() + "/" + &SRC_FOLDER.to_owned())?;
    let mut bevy_lib_file =
        File::create(BEVY_FOLDER.to_owned() + "/" + &SRC_FOLDER.to_owned() + "/main.rs")?;
    bevy_lib_file.write_all(model.scope.to_string().as_bytes())?;

    let mut cargo_file = File::create(BEVY_FOLDER.to_owned() + "/Cargo.toml")?;
    cargo_file.write_all(
        r#"[package]
name = "bevy_game"
version = "0.1.0"
edition = "2021"

# Enable only a small amount of optimization in debug mode
[profile.dev]
opt-level = 1

# Enable high optimizations for dependencies (incl. Bevy), but not for our code:
[profile.dev.package."*"]
opt-level = 3

[dependencies.bevy]
version = "0.6"
default-features = false
"#
        .as_bytes(),
    )?;
    Ok(())
}
