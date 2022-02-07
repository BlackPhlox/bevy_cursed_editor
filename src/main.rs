extern crate codegen;
use codegen::{Function, Scope, Struct};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
};

#[derive(Serialize, Deserialize, Clone, Default)]
struct BevyModel {
    plugins: Vec<Plugin>,
    components: Vec<Component>,
    startup_systems: Vec<System>,
    systems: Vec<System>,
    bevy_settings: Settings,
    model_meta: Meta,
    examples: Vec<BevyModel>,
}

impl BevyModel {
    fn generate(&self) -> Scope {
        let mut scope = Scope::new();

        scope.import("bevy::prelude", "*");

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

        match &self.model_meta.bevy_type {
            BevyType::Plugin(s) => scope.create_plugin(s, false, &app_code_merge),
            BevyType::PluginGroup(s) => scope.create_plugin(s, true, &app_code_merge),
            _ => scope.create_app(&app_code_merge),
        };

        for component in &self.components {
            scope.create_component(&component.name);
        }

        for system in &self.startup_systems {
            scope.create_query(&system.name, &system.content);
        }
        for system in &self.systems {
            scope.create_query(&system.name, &system.content);
        }
        scope
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
enum BevyType {
    App,
    Plugin(String),
    PluginGroup(String),
    Example,
}
#[derive(Serialize, Deserialize, Clone)]
struct Meta {
    name: String,
    bevy_type: BevyType,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            name: "bevy_default_meta".to_string(),
            bevy_type: BevyType::App,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct System {
    name: String,
    content: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Component {
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Plugin {
    name: String,
    is_group: bool,
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct Settings {
    features: Vec<Feature>,
    dev_features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Clone)]
enum Feature {
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

    fn create_query(&mut self, name: &str, content: &str) -> &mut Function;

    fn create_component(&mut self, name: &str) -> &mut Struct;
}
impl BevyCodegen for Scope {
    fn create_app(&mut self, content: &str) -> &mut Function {
        self.raw("#[bevy_main]")
            .new_fn("main")
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

    fn create_query(&mut self, name: &str, content: &str) -> &mut Function {
        self.new_fn(name).line(content)
    }

    fn create_component(&mut self, name: &str) -> &mut Struct {
        self.new_struct(name).derive("Component")
    }
}

fn main() {
    let mut bevy_model = BevyModel {
        model_meta: Meta {
            name: "bevy_test".to_string(),
            bevy_type: BevyType::Plugin("BevyTest".to_string()),
        },
        examples: vec![BevyModel {
            model_meta: Meta {
                name: "example_test".to_string(),
                bevy_type: BevyType::Example,
            },
            ..Default::default()
        }],
        ..Default::default()
    };

    bevy_model.bevy_settings.features.push(Feature::Render);

    bevy_model.plugins.push(Plugin {
        name: "DefaultPlugins".to_string(),
        is_group: true,
    });

    bevy_model.components.push(Component {
        name: "Test1".to_string(),
    });

    let hw_system = System {
        name: "hello_world".to_string(),
        content: "println!(\"Hello World!\");".to_string(),
    };
    bevy_model.startup_systems.push(hw_system);

    let scope = bevy_model.generate();

    println!("{}", scope.to_string());

    let serialized = serde_json::to_string(&bevy_model).unwrap();
    println!("serialized = {}", serialized);

    let _ = write_to_file(bevy_model.clone());

    build_and_run(bevy_model);
}

fn build_and_run(model: BevyModel) {
    let path = model.model_meta.name;
    println!("fmt");
    let _fmt = Command::new("cargo")
        .arg("fmt")
        .arg("--all")
        .current_dir(path.clone())
        .status() //output()
        .expect("failed to execute cargo fmt");

    println!("update");
    let _clippy = Command::new("cargo")
        .arg("update")
        .current_dir(path.clone())
        .status() //output()
        .expect("failed to execute cargo update");

    println!("build");
    let _clippy = Command::new("cargo")
        .arg("build")
        .current_dir(path.clone())
        .status() //output()
        .expect("failed to execute cargo build");

    println!("fix");
    let _fix = Command::new("cargo")
        .arg("clippy")
        .arg("--fix")
        .arg("--allow-no-vcs")
        .current_dir(path.clone())
        .status() //output()
        .expect("failed to execute cargo fix");

    println!("clippy");
    let _clippy = Command::new("cargo")
        .arg("clippy")
        .arg("--")
        .arg("-D")
        .arg("warnings")
        .current_dir(path.clone())
        .status() //output()
        .expect("failed to execute cargo clippy");

    if let BevyType::App = model.model_meta.bevy_type {
        println!("run");
        let _run = Command::new("cargo")
            .arg("run")
            .current_dir(path.clone())
            .status() //output()
            .expect("failed to execute cargo run");
    }

    println!("example(s)");
    for example in model.examples {
        let _run = Command::new("cargo")
            .arg("run")
            .arg("--example")
            .arg(example.model_meta.name)
            .current_dir(path.clone())
            .status() //output()
            .expect("failed to execute cargo run");
    }

    /*
    Open generated project in VSCode
    println!("code");
    let _code = Command::new("code")
        .arg(".")
        .current_dir(path)
        .status() //output()
        .expect("failed to open vscode");
    */
}

fn feature_write(features: Vec<Feature>) -> String {
    let mut features_str = "".to_owned();
    if features.is_empty() {
        features_str.push_str("default-features = false");
    } else {
        features_str += "features = [";
        let len = features.len();
        for (i, feature) in features.into_iter().enumerate() {
            features_str += format!("\"{}\"", feature.to_feature()).as_str();
            if i != len - 1 {
                features_str += ",";
            }
        }
        features_str += "]";
    }
    features_str
}

fn write_to_file(model: BevyModel) -> std::io::Result<()> {
    let bevy_folder = model.model_meta.name.clone();
    const SRC_FOLDER: &str = "src";
    if Path::new(&bevy_folder).exists() {
        fs::remove_dir_all(bevy_folder.to_owned() + "/" + &SRC_FOLDER.to_owned())?;
        let _rf = fs::remove_file(bevy_folder.to_owned() + "/" + "Cargo.toml");
    } else {
        fs::create_dir(bevy_folder.to_owned())?;
    }
    fs::create_dir(bevy_folder.to_owned() + "/" + &SRC_FOLDER.to_owned())?;

    //Write plugin or main/game
    let bevy_type_filename = match model.model_meta.bevy_type {
        BevyType::App => "/main.rs",
        _ => "/lib.rs",
    };
    let mut bevy_lib_file =
        File::create(bevy_folder.to_owned() + "/" + &SRC_FOLDER.to_owned() + bevy_type_filename)?;
    let _ = bevy_lib_file
        .write("#![cfg_attr(not(debug_assertions), windows_subsystem = \"windows\")]".as_bytes());
    bevy_lib_file.write_all(model.generate().to_string().as_bytes())?;

    //Write examples
    if !model.examples.is_empty() {
        fs::create_dir(bevy_folder.to_owned() + "/" + "examples")?;
        for example in model.examples {
            let mut bevy_example_file = File::create(
                bevy_folder.to_owned() + "/examples/" + &example.model_meta.name + ".rs",
            )?;
            bevy_example_file.write_all(example.generate().to_string().as_bytes())?;
        }
    }

    //Write cargo toml
    let mut cargo_file = File::create(bevy_folder.to_owned() + "/Cargo.toml")?;

    let features = feature_write(model.bevy_settings.features);
    let dev_features = feature_write(model.bevy_settings.dev_features);

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
winit = {{ version = "0.25", features=["x11"]}}

[dependencies.bevy]
version = "0.6"
{features}

[dev-dependencies.bevy]
version = "0.6"
{dev_features}

"#,
        meta_name = bevy_folder,
        features = features,
        dev_features = dev_features,
    );

    cargo_file.write_all(buf.as_bytes())?;
    Ok(())
}
