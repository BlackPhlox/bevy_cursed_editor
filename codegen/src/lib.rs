extern crate codegen;
use codegen::{Function, Scope, Struct};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct BevyModel {
    pub plugins: Vec<Plugin>,
    pub components: Vec<Component>,
    pub startup_systems: Vec<System>,
    pub systems: Vec<System>,
    pub bevy_settings: Settings,
    pub meta: Meta,
    pub examples: Vec<BevyModel>,
}

/*
impl Display for BevyModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "{:?}", &self);
        Ok(())
    }
}
*/

impl Display for BevyModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "BevyModel:");

        let _ = writeln!(f, "   Meta:");

        let _ = writeln!(f, "       {:?}", &self.meta);

        let _ = writeln!(f, "   Components:");
        (&self.components).iter().for_each(|d| {
            let _ = writeln!(f, "       {}", d.name);
        });

        let _ = writeln!(f);

        let _ = writeln!(f, "   Startup Systems:");
        (&self.startup_systems).iter().for_each(|d| {
            let _ = writeln!(f, "       {}", d.name);
        });

        let _ = writeln!(f);

        let _ = writeln!(f, "   Runtime Systems:");
        (&self.systems).iter().for_each(|d| {
            let _ = writeln!(f, "       {},", d.name);
        });

        let _ = writeln!(f);

        let _ = writeln!(f, "   Plugins:");
        (&self.plugins).iter().for_each(|d| {
            let _ = writeln!(f, "       {:?},", d);
        });

        Ok(())
    }
}

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
            scope.create_query(&system.name, system.param.clone(), &system.content);
        }
        for system in &self.systems {
            scope.create_query(&system.name, system.param.clone(), &system.content);
        }
        scope
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub enum BevyType {
    App,
    Plugin(String),
    PluginGroup(String),
    Example,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Meta {
    pub name: String,
    pub bevy_type: BevyType,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            name: "bevy_default_meta".to_string(),
            bevy_type: BevyType::App,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct System {
    pub name: String,
    pub param: Vec<(String, String)>,
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Component {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Plugin {
    pub name: String,
    pub is_group: bool,
    pub dependencies: Vec<PluginDependency>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct PluginDependency {
    pub crate_name: String,
    pub crate_version: String,
    pub crate_paths: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Settings {
    pub features: Vec<Feature>,
    pub dev_features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Feature {
    Default,
    BevyAudio,
    BevyGilrs,
    BevyWinit,
    Render,
    Png,
    Hdr,
    Vorbis,
    X11,
    FilesystemWatcher,
    TraceChrome,
    TraceTracy,
    Wayland,
    WgpuTrace,
    BevyCiTesting,
    BevySprite,
    Dynamic,
    BevyUi,
    Tga,
    Serialize,
    Mp3,
    BevyCorePipeline,
    Wav,
    Trace,
    SubpixelGlyphAtlas,
    Bmp,
    BevyGltf,
    Dds,
    BevyDynamicPlugin,
    BevyRender,
    BevyText,
    Flac,
    BevyPbr,
    Jpeg,
    BevyDylib,
}

impl Feature {
    fn to_feature(&self) -> &'static str {
        match self {
            Feature::Default => "default",
            Feature::BevyAudio => "bevy_audio",
            Feature::BevyGilrs => "bevy_gilrs",
            Feature::BevyWinit => "bevy_winit",
            Feature::Render => "render",
            Feature::Png => "png",
            Feature::Hdr => "hdr",
            Feature::Vorbis => "vorbis",
            Feature::X11 => "x11",
            Feature::FilesystemWatcher => "filesystem_watcher",
            Feature::TraceChrome => "trace_chrome",
            Feature::TraceTracy => "trace_tracy",
            Feature::Wayland => "wayland",
            Feature::WgpuTrace => "wgpu_trace",
            Feature::BevyCiTesting => "bevy_ci_testing",
            Feature::BevySprite => "bevy_sprite",
            Feature::Dynamic => "dynamic",
            Feature::BevyUi => "bevy_ui",
            Feature::Tga => "tga",
            Feature::Serialize => "serialize",
            Feature::Mp3 => "mp3",
            Feature::BevyCorePipeline => "bevy_core_pipeline",
            Feature::Wav => "wav",
            Feature::Trace => "trace",
            Feature::SubpixelGlyphAtlas => "subpixel_glyph_atlas",
            Feature::Bmp => "bmp",
            Feature::BevyGltf => "bevy_gltf",
            Feature::Dds => "dds",
            Feature::BevyDynamicPlugin => "bevy_dynamic_plugin",
            Feature::BevyRender => "bevy_render",
            Feature::BevyText => "bevy_text",
            Feature::Flac => "flac",
            Feature::BevyPbr => "bevy_pbr",
            Feature::Jpeg => "jpeg",
            Feature::BevyDylib => "bevy_dylib",
        }
    }
}

trait BevyCodegen {
    fn create_app(&mut self, content: &str) -> &mut Function;

    fn create_plugin(&mut self, name: &str, is_group: bool, content: &str) -> &mut Function;

    fn create_query(
        &mut self,
        name: &str,
        args: Vec<(String, String)>,
        content: &str,
    ) -> &mut Function;

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

    fn create_query(
        &mut self,
        name: &str,
        args: Vec<(String, String)>,
        content: &str,
    ) -> &mut Function {
        let mut a = self.new_fn(name);
        for (name, ty) in args {
            a = a.arg(&name, ty);
        }
        a.line(content)
    }

    fn create_component(&mut self, name: &str) -> &mut Struct {
        self.new_struct(name).derive("Component")
    }
}

pub fn create_default_template() -> BevyModel {
    let mut bevy_model = BevyModel {
        meta: Meta {
            name: "bevy_test".to_string(),
            bevy_type: BevyType::App,
        },
        ..Default::default()
    };

    bevy_model.components.push(Component {
        name: "Test1".to_string(),
    });

    let hw_system = System {
        name: "hello_world".to_string(),
        param: Vec::new(),
        content: "println!(\"Hello World!\");".to_string(),
    };
    bevy_model.startup_systems.push(hw_system);

    bevy_model.bevy_settings.features.push(Feature::Dynamic);

    bevy_model
}

pub fn create_plugin_template() -> BevyModel {
    let mut bevy_model = BevyModel {
        meta: Meta {
            name: "bevy_test".to_string(),
            bevy_type: BevyType::Plugin("BevyTest".to_string()),
        },
        examples: vec![BevyModel {
            meta: Meta {
                name: "example_test".to_string(),
                bevy_type: BevyType::Example,
            },
            plugins: vec![Plugin {
                name: "BevyTest".to_string(),
                is_group: false,
                dependencies: vec![],
            }],
            ..Default::default()
        }],
        ..Default::default()
    };

    /*bevy_model.bevy_settings.features.push(Feature::Render);

    bevy_model.plugins.push(Plugin {
        name: "DefaultPlugins".to_string(),
        is_group: true,
    });*/

    bevy_model.components.push(Component {
        name: "Test1".to_string(),
    });

    let hw_system = System {
        name: "hello_world".to_string(),
        param: Vec::new(),
        content: "println!(\"Hello World From Plugin!\");".to_string(),
    };
    bevy_model.startup_systems.push(hw_system);

    bevy_model
}

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
winit = {{ version = "0.26.1", features=["x11"]}}

[dependencies]
{crate_deps}
[dependencies.bevy]
version = "0.7"
{features}

[dev-dependencies.bevy]
version = "0.7"
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
