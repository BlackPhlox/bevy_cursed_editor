use crate::{BevyModel, BevyType};
use std::process::Command;

pub fn cmd_fmt(model: BevyModel) {
    let path = model.meta.name;
    println!("fmt");
    let _fmt = Command::new("cargo")
        .arg("fmt")
        .arg("--all")
        .current_dir(path)
        .status() //output()
        .expect("failed to execute cargo fmt");
}

pub fn cmd_build(model: BevyModel) {
    cmd_fmt(model.clone());
    let path = model.meta.name;

    println!("update");
    let _update = Command::new("cargo")
        //.arg("+nightly")
        .arg("update")
        .current_dir(path.clone())
        .status() //output()
        .expect("failed to execute cargo update");

    println!("build");
    let _build = Command::new("cargo")
        //.arg("+nightly")
        .arg("build")
        .current_dir(path.clone())
        .status() //output()
        .expect("failed to execute cargo build");

    println!("fix");
    let _fix = Command::new("cargo")
        //.arg("+nightly")
        .arg("clippy")
        .arg("--fix")
        .arg("--allow-no-vcs")
        .current_dir(path.clone())
        .status() //output()
        .expect("failed to execute cargo fix");

    println!("clippy");
    let _clippy = Command::new("cargo")
        //.arg("+nightly")
        .arg("clippy")
        .arg("--")
        .arg("-D")
        .arg("warnings")
        .current_dir(path)
        .status() //output()
        .expect("failed to execute cargo clippy");
}

pub fn cmd_default(model: BevyModel, spawn: bool) {
    cmd_build(model.clone());
    let path = model.meta.name;

    if let BevyType::App = model.meta.bevy_type {
        println!("run");
        if spawn {
            let _run = Command::new("cargo")
                //.arg("+nightly")
                .arg("run")
                .current_dir(path.clone())
                .spawn();
        } else {
            let _run = Command::new("cargo")
                //.arg("+nightly")
                .arg("run")
                .current_dir(path.clone())
                .status() //output()
                .expect("failed to execute cargo run");
        }
    }

    println!("example(s)");
    for example in model.examples {
        println!("Running {}", example.meta.name);
        let _run = Command::new("cargo")
            //.arg("+nightly")
            .arg("run")
            .arg("--example")
            .arg(example.meta.name)
            .current_dir(path.clone())
            .status() //output()
            .expect("failed to execute cargo run");
    }
}

pub fn cmd_code(model: BevyModel) {
    let path = model.meta.name;
    //Open generated project in VSCode
    println!("code");
    let _code = Command::new("code")
        .arg(".")
        .current_dir(path)
        .status() //output()
        .expect("failed to open vscode");
}

pub fn cmd_clean(_model: BevyModel) {
    println!("clean");
}

pub fn cmd_release(_model: BevyModel) {
    println!("release");
}
