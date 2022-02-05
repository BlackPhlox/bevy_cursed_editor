extern crate codegen;
use codegen::Scope;
use std::{
    fmt::Debug,
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
};

struct BevyModel {
    scope: Scope,
}

fn main() {
    let mut bevy_model = BevyModel {
        scope: Scope::new(),
    };

    bevy_model.scope.import("bevy::prelude", "*");

    bevy_model
        .scope
        .new_struct("Foo")
        .derive("Debug")
        .field("one", "usize")
        .field("two", "String");

    bevy_model.scope.raw("#[bevy_main]");
    bevy_model
        .scope
        .new_fn("main")
        .line("App::new().add_system(hello_world_system).run();");

    bevy_model
        .scope
        .new_fn("hello_world_system")
        .line("println!(\"hello world\");");

    println!("{}", bevy_model.scope.to_string());

    let _ = write_to_file(bevy_model);

    //Action 1
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
    };
    let stdout = String::from_utf8(output.stdout).unwrap();
    println!("{:?}", stdout);

    println!("fmt");
    let fmt = Command::new("cargo")
        .arg("fmt")
        .arg("--all")
        .current_dir("bevy")
        .status() //output()
        .expect("failed to execute cargo fmt");

    println!("fix");
    let fix = Command::new("cargo")
        .arg("clippy")
        .arg("--fix")
        .arg("--allow-no-vcs")
        .current_dir("bevy")
        .status() //output()
        .expect("failed to execute cargo fix");

    println!("clippy");
    let clippy = Command::new("cargo")
        .arg("clippy")
        .arg("--")
        .arg("-D")
        .arg("warnings")
        .current_dir("bevy")
        .status() //output()
        .expect("failed to execute cargo clippy");

    println!("{:?}", clippy);

    println!("run");
    let run = Command::new("cargo")
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
        fs::remove_file(BEVY_FOLDER.to_owned() + "/" + "Cargo.toml");
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
