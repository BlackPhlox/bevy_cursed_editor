extern crate codegen;
use codegen::{Function, Scope, Struct};
use model::{BevyModel, BevyType, Feature};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub mod commands;
pub mod model;
pub mod templates;

impl BevyModel {
    pub fn generate(&self) -> Scope {
        let mut scope = Scope::new();

        if self.meta.bevy_type.eq(&BevyType::Example) {
            scope.import("bevy_test", "BevyTest");
        }

        let mut plugin_app_code: String = "".into();
        for plugin in &self.plugins {
            if plugin.is_group {
                plugin_app_code.push_str(format!(".add_plugins({})", &plugin.name).as_str());
            } else {
                plugin_app_code.push_str(format!(".add_plugin({})", &plugin.name).as_str());
            }
        }

        let mut startup_system_app_code: String = "".into();
        for system in &self.startup_systems {
            startup_system_app_code
                .push_str(format!(".add_startup_system({})", &system.name).as_str());
        }

        let mut system_app_code: String = "".into();
        for system in &self.systems {
            system_app_code.push_str(format!(".add_system({})", &system.name).as_str());
        }

        let mut app_code_merge: String = "".to_owned();
        app_code_merge.push_str(&plugin_app_code);
        app_code_merge.push_str(&startup_system_app_code);
        app_code_merge.push_str(&system_app_code);

        match &self.meta.bevy_type {
            BevyType::Plugin(name) => scope.create_plugin(name, false, &app_code_merge),
            BevyType::PluginGroup(name) => scope.create_plugin(name, true, &app_code_merge),
            _ => scope.create_app(&app_code_merge),
        };

        for component in &self.components {
            scope.create_component(&component.name);
        }

        for system in &self.startup_systems {
            scope.create_query(&system);
        }
        for system in &self.systems {
            scope.create_query(&system);
        }
        scope
    }
}

trait BevyCodegen {
    fn create_app(&mut self, content: &str) -> &mut Function;

    fn create_plugin(&mut self, name: &str, is_group: bool, content: &str) -> &mut Function;

    fn create_query(&mut self, system: &crate::model::System) -> &mut Function;

    fn create_component(&mut self, name: &str) -> &mut Struct;
}
impl BevyCodegen for Scope {
    fn create_app(&mut self, content: &str) -> &mut Function {
        self.new_fn("main")
            .line(format!("App::new(){}.run();", content))
    }

    fn create_plugin(&mut self, name: &str, is_group: bool, content: &str) -> &mut Function {
        self.new_struct(name).vis("pub");
        let plugin_impl = match is_group {
            false => self.new_impl(name).impl_trait("Plugin"),
            true => self.new_impl(name).impl_trait("Plugins"),
        };
        plugin_impl
            .new_fn("build")
            .arg_ref_self()
            .arg("app", "&mut App")
            .line("app")
            .line(content)
            .line(";")
    }

    fn create_query(&mut self, system: &crate::model::System) -> &mut Function {
        let mut fun = self.new_fn(system.name.as_str());
        for (name, ty) in &system.param {
            fun = fun.arg(&name, ty);
        }
        fun.vis(&system.visibility);
        for att in &system.attributes {
            fun.attr(att);
        }
        fun.line(system.content.clone())
    }

    fn create_component(&mut self, name: &str) -> &mut Struct {
        self.new_struct(name).derive("Component")
    }
}

pub fn feature_write(features: &Vec<Feature>) -> String {
    let mut features_str = "".to_owned();
    if features.is_empty() {
        features_str.push_str("default-features = false");
    } else {
        features_str += "features = [";
        let len = features.len();
        for (i, feature) in features.iter().enumerate() {
            features_str += format!("\"{}\"", feature.to_feature()).as_str();
            if i != len - 1 {
                features_str += ",";
            }
        }
        features_str += "]";
    }
    features_str
}

pub fn write_to_file(model: BevyModel) -> std::io::Result<()> {
    let bevy_folder = model.meta.name.clone();
    const SRC_FOLDER: &str = "src";
    const CONFIG_FOLDER: &str = ".cargo";
    if Path::new(&bevy_folder).exists() {
        fs::remove_dir_all(bevy_folder.to_owned() + "/" + SRC_FOLDER)?;
        fs::remove_dir_all(bevy_folder.to_owned() + "/" + CONFIG_FOLDER)?;
        let _rf = fs::remove_file(bevy_folder.to_owned() + "/Cargo.toml");
    } else {
        fs::create_dir(&bevy_folder)?;
    }

    //Write cargo toml
    let mut cargo_file = File::create(bevy_folder.to_owned() + "/Cargo.toml")?;

    let features = feature_write(&model.bevy_settings.features);
    let dev_features = feature_write(&model.bevy_settings.dev_features);

    let crate_deps = model
        .plugins
        .iter()
        .map(|d| {
            let mut s = "".to_owned();
            for b in d.dependencies.iter() {
                let k = if b.crate_version.starts_with('{') {
                    format!("{} = {}\n", b.crate_name, b.crate_version)
                } else {
                    format!("{} = \"{}\"\n", b.crate_name, b.crate_version)
                };
                s.push_str(&k);
            }
            s.to_string()
        })
        .collect::<Vec<String>>()
        .join("");

    let buf = format!(
        r#"[package]
name = "{meta_name}"
version = "0.1.0"
edition = "2021"

[workspace]

# Enable only a small amount of optimization in debug mode
[profile.dev]
opt-level = 1

# Enable high optimizations for dependencies (incl. Bevy), but not for our code:
[profile.dev.package."*"]
opt-level = 3

# Maximize release performance with Link-Time-Optimization
[profile.release]
lto = "thin"
codegen-units = 1

[target.'cfg(target_os = "linux")'.dependencies]
winit = {{ version = "0.27", features=["x11"]}}

[dependencies]
{crate_deps}
[dependencies.bevy]
version = "0.8"
{features}

[dev-dependencies.bevy]
version = "0.8"
{dev_features}

"#,
        meta_name = bevy_folder,
        features = features,
        dev_features = dev_features,
    );

    cargo_file.write_all(buf.as_bytes())?;

    fs::create_dir(bevy_folder.to_owned() + "/" + CONFIG_FOLDER)?;

    let mut cargo_config_file =
        File::create(bevy_folder.to_owned() + "/" + CONFIG_FOLDER + "/config.toml")?;
    let ccf_buf = r#"[target.x86_64-pc-windows-msvc]
linker = "rust-lld.exe"
rustflags = ["-Zshare-generics=off"]"#;

    cargo_config_file.write_all(ccf_buf.as_bytes())?;

    fs::create_dir(bevy_folder.to_owned() + "/" + SRC_FOLDER)?;

    //Write plugin or main/game
    let bevy_type_filename = match model.meta.bevy_type {
        BevyType::App => "/main.rs",
        _ => "/lib.rs",
    };
    let mut bevy_lib_file =
        File::create(bevy_folder.to_owned() + "/" + SRC_FOLDER + bevy_type_filename)?;

    let import_deps = model
        .plugins
        .iter()
        .map(|d| {
            let mut s = "".to_owned();
            for b in d.dependencies.iter() {
                for c in b.crate_paths.iter() {
                    s.push_str(&format!("use {}::{};\n", b.crate_name, c));
                }
            }
            s.to_string()
        })
        .collect::<Vec<String>>()
        .join("");

    let _ = bevy_lib_file
        .write("#![cfg_attr(not(debug_assertions), windows_subsystem = \"windows\")]\nuse bevy::prelude::*;\n".as_bytes());

    let _ = bevy_lib_file.write((import_deps + "\n").as_bytes());

    if model.meta.bevy_type.eq(&BevyType::App) {
        let _ = bevy_lib_file.write(("#[bevy_main]\n").as_bytes());
    }

    bevy_lib_file.write_all(model.generate().to_string().as_bytes())?;

    //Write examples
    fs::remove_dir_all(bevy_folder.to_owned() + "/examples")?;
    if !model.examples.is_empty() {
        fs::create_dir(bevy_folder.to_owned() + "/" + "examples")?;
        for example in model.examples {
            let mut bevy_example_file =
                File::create(bevy_folder.to_owned() + "/examples/" + &example.meta.name + ".rs")?;
            bevy_example_file.write_all(example.generate().to_string().as_bytes())?;
        }
    }

    Ok(())
}
