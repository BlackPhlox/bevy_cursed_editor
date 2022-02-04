extern crate codegen;
use std::{process::Command, fs::{File, self}, io::Write};
use codegen::Scope;

struct BevyModel{
    scope: Scope
}

fn main() {
    



    let mut bevy_model = BevyModel{scope: Scope::new()};

    bevy_model.scope.new_struct("Foo")
        .derive("Debug")
        .field("one", "usize")
        .field("two", "String");
    
    bevy_model.scope.raw("#[bevy_main]");
    bevy_model.scope.new_fn("main");

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

    //Action 2
    /*let mut output2 = Command::new("cd")
        .arg("-l")
        .arg("-a")
        .spawn()
        .expect("ls command failed to start");*/
    //let stdout2 = String::from_utf8(output2.stdout).unwrap();
    //println!("{:?}", stdout2);
    let cmd = Command::new("cargo").spawn().unwrap().wait_with_output();
    println!("{:?}", cmd);
}

fn write_to_file(model : BevyModel) -> std::io::Result<()>{
    const BEVY_FOLDER: &str = "bevy";
    const SRC_FOLDER: &str = "src";
    fs::remove_dir_all(BEVY_FOLDER)?;
    fs::create_dir(BEVY_FOLDER)?;
    fs::create_dir(BEVY_FOLDER.to_owned() + "/" + &SRC_FOLDER.to_owned())?;
    let mut bevy_lib_file = File::create(BEVY_FOLDER.to_owned() + "/" + &SRC_FOLDER.to_owned() + "/lib.rs")?;
    bevy_lib_file.write_all(model.scope.to_string().as_bytes())?;

    let mut cargo_file = File::create(BEVY_FOLDER.to_owned() + "/Cargo.toml")?;
    cargo_file.write_all(r#"[package]
name = "bevy_game"
version = "0.1.0"
edition = "2021""#
    .as_bytes())?;
    Ok(())
}
