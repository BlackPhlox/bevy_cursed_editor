//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_codegen::{
    commands::{cmd_default, cmd_fmt},
    model::{BevyModel, BevyType, Feature, Meta, PluginDependency, Settings},
    templates::{default_game::create_default_template, default_plugin::create_plugin_template},
    write_to_file,
};
use bevy_editor_pls::{
    default_windows::hierarchy::picking::EditorRayCastSource,
    editor_window::{EditorWindow, EditorWindowContext},
    prelude::*, egui,
};
use bevy_egui::EguiPlugin;
use undo::{Action, History};
//use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};
//use bevy_transform_gizmo::TransformGizmoPlugin;

pub fn start_editor() {
    App::new()
        //.init_resource::<GameModel>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        //.add_plugin(EguiPlugin)
        //.add_plugin(InfiniteGridPlugin)
        .add_editor_window::<CursedOverviewWindow>()
        .add_editor_window::<CursedEntitiesWindow>()
        .add_editor_window::<CursedComponentsWindow>()
        .add_editor_window::<CursedSystemsWindow>()
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_startup_system(setup_non_send.label("one"))
        .add_startup_system(setup.after("one"))
        .run();
}

fn setup_non_send(world: &mut World) {
    let mut target = ProjectModel {
        model: BevyModel {
            plugins: vec![],
            components: vec![],
            startup_systems: vec![],
            systems: vec![],
            bevy_settings: Settings {
                features: vec![],
                dev_features: vec![],
            },
            meta: Meta {
                name: "bevy_test".to_string(),
                bevy_type: BevyType::App,
            },
            examples: vec![],
        },
        history: History::new()
    };
    
    target.apply(Add(BevyModelAction::Component("Test1".to_string(), vec![])));

    println!("{:?}", target);
    
    world.insert_non_send_resource(target);
}

impl ProjectModel {
    fn apply(&mut self, add : Add){
        self.history.apply(&mut self.model, add);
    }

    fn redo(&mut self) {
        self.history.redo(&mut self.model);
    }

    fn undo(&mut self) {
        self.history.undo(&mut self.model);
    }
}

#[derive(Debug)]
pub struct ProjectModel {
    model: BevyModel,
    history: History<Add>
}

#[derive(Clone, Debug)]
enum BevyModelAction {
    Component(String, Vec<(String, String)>),
}

#[derive(Clone, Debug)]
struct Add(BevyModelAction);

impl Action for Add {
    type Target = BevyModel;
    type Output = ();
    type Error = &'static str;

    fn apply(&mut self, s: &mut Self::Target) -> undo::Result<Add> {
        match &self.0 {
            BevyModelAction::Component(x, y) => {
                s.components.push(bevy_codegen::model::Component {
                    name: x.to_string(),
                    content: y.to_vec(),
                });
            }
        };
        Ok(())
    }

    fn undo(&mut self, s: &mut Self::Target) -> undo::Result<Add> {
        match self.0 {
            BevyModelAction::Component(_, _) => s.components.pop(),
        };
        Ok(())
    }
}

impl Default for ProjectModel {
    fn default() -> Self {
        Self {
            model: create_default_template_v2(),
            history: History::new()
        }
    }
}

pub fn create_default_template_v2() -> BevyModel {
    let mut bevy_model = BevyModel {
        meta: Meta {
            name: "bevy_test".to_string(),
            bevy_type: BevyType::App,
        },
        ..default()
    };

    bevy_model.components.push(bevy_codegen::model::Component {
        name: "Test1".to_string(),
        content: vec![],
    });

    bevy_model.plugins.push(bevy_codegen::model::Plugin {
        name: "DefaultPlugins".to_string(),
        is_group: true,
        dependencies: vec![],
    });

    /*bevy_model.plugins.push(bevy_codegen::model::Plugin {
        name: "ConfigCam".to_string(),
        is_group: false,
        dependencies: vec![PluginDependency {
            crate_name: "bevy_config_cam".into(),
            crate_version: "0.3.0".into(),
            crate_paths: vec!["*".into()],
        }],
    });*/

    let hw_system = bevy_codegen::model::System {
        name: "setup".to_string(),
        param: vec![
            ("mut commands".to_string(), "Commands".to_string()),
            ("mut meshes".to_string(), "ResMut<Assets<Mesh>>".to_string()),
            (
                "mut materials".to_string(),
                "ResMut<Assets<StandardMaterial>>".to_string(),
            ),
        ],
        visibility: "pub".to_string(),
        attributes: vec![],
        content: r#"
        // plane
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        });
        // cube
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        });
        // light
        commands.spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        });
        // camera
        commands.spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });"#
            .to_string(),
    };
    bevy_model.startup_systems.push(hw_system);

    bevy_model.bevy_settings.features.push(Feature::Dynamic);

    let scope = bevy_model.generate();

    println!("{}", scope.to_string());

    let _ = write_to_file(bevy_model.clone());

    bevy_model
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut camera: Query<(Entity, &mut Camera), With<Camera3d>>,
) {

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    //let mut a = camera.iter().count();

    /*for (e, mut c) in &mut camera {
        /*commands
        .entity(e)
        .insert_bundle(bevy_mod_picking::PickingCameraBundle::default())
        .insert(bevy_transform_gizmo::GizmoPickSource::default());*/
        if a > 0 {
            c.priority = a as isize;
            a -= 1;
        } else {
            c.is_active = true;
        }
    }*/

    // camera
    /*commands
    .spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(EditorRayCastSource::new());*/
    /* .insert_bundle(bevy_mod_picking::PickingCameraBundle::default())
    .insert(bevy_transform_gizmo::GizmoPickSource::default());*/

    //commands.spawn_bundle(InfiniteGridBundle::default());
}

/*
fn main() {
    let bevy_model = create_default_template();
    //let bevy_model =create_plugin_template();

    let scope = bevy_model.generate();

    println!("{}", scope.to_string());

    let serialized = serde_json::to_string(&bevy_model).unwrap();
    println!("serialized = {}", serialized);

    let _ = write_to_file(bevy_model.clone());

    //cmd_clean(bevy_model.clone());
    cmd_default(bevy_model.clone());
    //cmd_release(bevy_model.clone());
    //cmd_code(bevy_model);
}
*/

pub struct CursedOverviewWindow;
impl EditorWindow for CursedOverviewWindow {
    type State = ();
    const NAME: &'static str = "Cursed Overview";

    fn ui(world: &mut World, _cx: EditorWindowContext, ui: &mut bevy_editor_pls::egui::Ui) {
        //let currently_inspected = cx.state::<HierarchyWindow>().unwrap().selected;

        ui.horizontal(|ui| {
            ui.menu_button("File", |ui| {
                ui.menu_button("New Project", |ui| {
                    if ui.button("Template App").clicked() {
                        let mut gm = world.get_non_send_resource_mut::<ProjectModel>().unwrap();
                        gm.model = create_default_template();
                        let _ = write_to_file(gm.model.clone());
                    }
                    if ui.button("Template Plugin").clicked() {
                        let mut gm = world.get_non_send_resource_mut::<ProjectModel>().unwrap();
                        gm.model = create_plugin_template();
                        let _ = write_to_file(gm.model.clone());
                    }
                });
                ui.label("Open Project");
                ui.label("Save Project");
                ui.label("Save As Project");
                if ui.button("Import Json").clicked() {
                    let mut gm = world.get_non_send_resource_mut::<ProjectModel>().unwrap();
                    let m = serde_json::from_str::<BevyModel>(
                        cli_clipboard::get_contents().unwrap().as_str(),
                    );
                    if let Ok(m) = m {
                        let _ = write_to_file(m.clone());
                        cmd_fmt(m.clone());
                        gm.model = m;
                    }
                }
                if ui.button("Export Json").clicked() {
                    let gm = world.get_non_send_resource_mut::<ProjectModel>().unwrap();
                    let m = gm.model.clone();
                    cli_clipboard::set_contents(
                        serde_json::to_string(&m).unwrap().replace("  ", ""),
                    )
                    .unwrap();
                }
                ui.label("Exit");
            });
            ui.menu_button("Edit", |ui| {
                let mut target = world.get_non_send_resource_mut::<ProjectModel>().unwrap();
                if ui.add_enabled(target.history.can_redo(), egui::Button::new("Redo")).clicked() {
                    target.redo();
                }
                if ui.add_enabled(target.history.can_undo(), egui::Button::new("Undo")).clicked() {
                    target.undo();
                }
                ui.label("History");
                ui.label("Project Settings");
            });

            ui.menu_button("Cargo", |ui| {
                if ui.button("Fmt").clicked() {
                    let gm = world.get_non_send_resource_mut::<ProjectModel>().unwrap();
                    cmd_fmt(gm.model.clone());
                }
                if ui.button("Run").clicked() {
                    let gm = world.get_non_send_resource_mut::<ProjectModel>().unwrap();
                    cmd_default(gm.model.clone(), true);
                }
            });
        });

        let gm = world.get_non_send_resource_mut::<ProjectModel>().unwrap();
        let m = gm.model.clone();
        ui.label(m.to_string());
    }
}

pub struct CursedEntitiesWindow;
impl EditorWindow for CursedEntitiesWindow {
    type State = ();
    const NAME: &'static str = "Cursed Entities";

    fn ui(world: &mut World, _cx: EditorWindowContext, ui: &mut bevy_editor_pls::egui::Ui) {
        //let currently_inspected = cx.state::<HierarchyWindow>().unwrap().selected;

        ui.label("Cursed Entities Overview");

        ui.menu_button("Entity", |ui| {
            ui.menu_button("Spawn using existing system", |ui| {
                let gm = world.get_non_send_resource_mut::<ProjectModel>().unwrap();
                let m = gm.model.clone();

                ui.label("Startup Systems:");
                m.startup_systems.iter().for_each(|s| {
                    if ui.button(s.name.as_str()).clicked() {
                        println!("Adding entity to startup system: {}", s.name);
                    }
                });

                ui.label("Runtime Systems:");
                m.systems.iter().for_each(|s| {
                    if ui.button(s.name.as_str()).clicked() {
                        println!("Adding entity to runtime system: {}", s.name);
                    }
                });
            });
            ui.menu_button("Spawn at startup", |ui| {
                if ui.button("Spawn Single").clicked() {
                    println!("Add Entity to new system startup");
                }
                if ui.button("Spawn Bundle").clicked() {
                    println!("Add Bundle to new system startup");
                }
            });

            ui.menu_button("Spawn at runtime", |ui| {
                if ui.button("Spawn Single").clicked() {
                    println!("Add Entity to new runtime startup");
                }
                if ui.button("Spawn Bundle").clicked() {
                    println!("Add Bundle to new runtime startup");
                }
            });
        });
    }
}

pub struct CursedComponentsWindow;
impl EditorWindow for CursedComponentsWindow {
    type State = ();
    const NAME: &'static str = "Cursed Components";

    fn ui(world: &mut World, _cx: EditorWindowContext, ui: &mut bevy_editor_pls::egui::Ui) {
        //let currently_inspected = cx.state::<HierarchyWindow>().unwrap().selected;

        ui.label("Cursed Components Overview");
        ui.menu_button("Component", |ui| {
            if ui.button("Create").clicked() {
                println!("Create component");
            }
            if ui.button("Add to entity").clicked() {
                println!("Add component to entity");
            }
        });
        let mut a: bool = true;
        ui.checkbox(&mut a, "Show project components only");
        ui.checkbox(&mut a, "Show used components only");
        let gm = world.get_non_send_resource_mut::<ProjectModel>().unwrap();
        let m = gm.model.clone();
        m.components.iter().for_each(|s| {
            ui.label(s.name.as_str());
        });
    }
}

pub struct CursedSystemsWindow;
impl EditorWindow for CursedSystemsWindow {
    type State = ();
    const NAME: &'static str = "Cursed Systems";

    fn ui(world: &mut World, _cx: EditorWindowContext, ui: &mut bevy_editor_pls::egui::Ui) {
        //let currently_inspected = cx.state::<HierarchyWindow>().unwrap().selected;

        ui.label("Cursed Systems Overview");
        ui.menu_button("System", |ui| {
            if ui.button("Add Startup").clicked() {
                println!("Add startup system");
            }
            if ui.button("Add Runtime").clicked() {
                println!("Add system");
            }
        });
        let gm = world.get_non_send_resource_mut::<ProjectModel>().unwrap();
        let m = gm.model.clone();
        m.startup_systems.iter().for_each(|s| {
            ui.label(s.name.as_str());
        });
        m.systems.iter().for_each(|s| {
            ui.label(s.name.as_str());
        });
    }
}
