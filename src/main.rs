extern crate codegen;
use codegen::{Function, Scope, Struct};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
};

#[derive(Serialize, Deserialize, Clone)]
struct BevyModel {
    plugins: Vec<Plugin>,
    components: Vec<Component>,
    startup_systems: Vec<System>,
    systems: Vec<System>,
    bevy_settings: Settings,
    model_meta: Meta,
}

impl BevyModel {
    fn generate(&mut self) -> Scope {
        let mut scope = Scope::new();

        scope.import("bevy::prelude", "*");

        let mut plugin_app_code: String = "".into();
        for plugin in &self.plugins {
            if plugin.is_group {
                plugin_app_code += format!(".add_plugins({})", &plugin.name).as_str();
            } else {
                plugin_app_code += format!(".add_plugin({})", &plugin.name).as_str();
            }
        }

        let mut startup_system_app_code: String = "".into();
        for system in &self.startup_systems {
            startup_system_app_code += format!(".add_startup_system({})", &system.name).as_str();
        }

        let mut system_app_code: String = "".into();
        for system in &self.systems {
            system_app_code += format!(".add_system({})", &system.name).as_str();
        }

        let mut app_code_merge: String = "".to_owned();
        app_code_merge.push_str(&plugin_app_code);
        app_code_merge.push_str(&startup_system_app_code);
        app_code_merge.push_str(&system_app_code);
        scope.create_app(&app_code_merge);

        for component in &self.components {
            scope.create_component(&component.name);
        }

        for system in &self.startup_systems {
            scope.create_query(&system.name, &system.base);
        }
        for system in &self.systems {
            scope.create_query(&system.name, &system.base);
        }
        scope
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Meta{
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct System {
    name: String,
    base: String,
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

#[derive(Serialize, Deserialize, Clone)]
struct Settings {
    features: Vec<Feature>,
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
    fn to_feature(self) -> &'static str {
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

    fn create_query(&mut self, name: &str, content: &str) -> &mut Function;

    fn create_component(&mut self, name: &str) -> &mut Struct;
}
impl BevyCodegen for Scope {
    fn create_app(&mut self, content: &str) -> &mut Function {
        self.raw("#[bevy_main]")
            .new_fn("main")
            .line(format!("App::new(){}.run();", content))
    }

    fn create_query(&mut self, name: &str, content: &str) -> &mut Function {
        self.new_fn(format!("{}", name).as_str()).line(content)
    }

    fn create_component(&mut self, name: &str) -> &mut Struct {
        self.new_struct(name)
    }
}

fn main() {
    let mut bevy_model = BevyModel {
        model_meta: Meta { name: "bevy_test".to_string() },
        plugins: vec![],
        components: vec![],
        startup_systems: vec![],
        systems: vec![],
        bevy_settings: Settings { features: vec![] },
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
        base: "println!(\"Hello World!\");".to_string(),
    };
    bevy_model.startup_systems.push(hw_system);

    let scope = bevy_model.generate();

    println!("{}", scope.to_string());

    let serialized = serde_json::to_string(&bevy_model).unwrap();
    println!("serialized = {}", serialized);

    let _ = write_to_file(bevy_model.clone());

    build_and_run(bevy_model);
}

fn build_and_run(mut model: BevyModel) {
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

    println!("run");
    let _run = Command::new("cargo")
        .arg("run")
        .current_dir(path)
        .status() //output()
        .expect("failed to execute cargo run");
}

fn write_to_file(mut model: BevyModel) -> std::io::Result<()> {
    let bevy_folder = model.model_meta.name.clone();
    const SRC_FOLDER: &str = "src";
    if Path::new(&bevy_folder).exists() {
        fs::remove_dir_all(bevy_folder.to_owned() + "/" + &SRC_FOLDER.to_owned())?;
        let _rf = fs::remove_file(bevy_folder.to_owned() + "/" + "Cargo.toml");
    } else {
        fs::create_dir(bevy_folder.to_owned())?;
    }
    fs::create_dir(bevy_folder.to_owned() + "/" + &SRC_FOLDER.to_owned())?;
    let mut bevy_lib_file =
        File::create(bevy_folder.to_owned() + "/" + &SRC_FOLDER.to_owned() + "/main.rs")?;
    bevy_lib_file
        .write("#![cfg_attr(not(debug_assertions), windows_subsystem = \"windows\")]".as_bytes())?;
    bevy_lib_file.write_all(model.generate().to_string().as_bytes())?;

    let mut cargo_file = File::create(bevy_folder.to_owned() + "/Cargo.toml")?;
    let buf = r#"[package]
    name = "bevy_game"
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
    winit = { version = "0.25", features=["x11"]}
    
    [dependencies.bevy]
    version = "0.6"
    "#;
    let mut buf2 = buf.to_owned();

    if model.bevy_settings.features.is_empty() {
        buf2 += "default-features = false";
    } else {
        buf2 += "features = [";
        let len = model.bevy_settings.features.len();
        for (i, feature) in model.bevy_settings.features.into_iter().enumerate() {
            buf2 += format!("\"{}\"", feature.to_feature()).as_str();
            if i == len {
                buf2 += ",";
            }
        }
        buf2 += "]";
    }

    cargo_file.write_all(buf2.as_bytes())?;
    Ok(())
}
