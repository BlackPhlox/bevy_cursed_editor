use crate::{
    model::{Component, Feature, Meta, Plugin, System},
    BevyModel, BevyType,
};

pub fn create_default_template() -> BevyModel {
    let mut bevy_model = BevyModel {
        meta: Meta {
            name: "bevy_test".to_string(),
            bevy_type: BevyType::App,
        },
        ..Default::default()
    };

    bevy_model.components.push(Component {
        name: "Player".to_string(),
        content: vec![
            ("velocity".to_string(), "Vec3".to_string()),
            ("rotation_speed".to_string(), "f32".to_string()),
            ("shooting_timer".to_string(), "Option<f32>".to_string()),
        ],
    });

    //For asset_server
    bevy_model.plugins.push(Plugin {
        name: "DefaultPlugins".to_string(),
        is_group: true,
        dependencies: vec![],
    });

    let setup_entities = System {
        name: "setup".to_string(),
        param: vec![
            ("mut commands".to_string(), "Commands".to_string()),
            ("asset_server".to_string(), "Res<AssetServer>".to_string()),
        ],
        content: r#"

commands.spawn_bundle(Camera2dBundle::default());

// player
let ship_handle = asset_server.load("ship_C.png");
commands
.spawn_bundle(SpriteBundle {
    texture: ship_handle,
    ..default()
})
.insert(Player {
    velocity: Vec3::ZERO,
    rotation_speed: f32::to_radians(180.0),
    shooting_timer: None,
});

"#
        .to_string(),
        visibility: "pub".to_string(),
        attributes: vec![],
    };

    bevy_model.startup_systems.push(setup_entities);

    let hw_system = System {
        name: "hello_world".to_string(),
        param: Vec::new(),
        content: "println!(\"Hello World!\");".to_string(),
        visibility: "pub".to_string(),
        attributes: vec!["no_mangle".to_string()],
    };
    bevy_model.startup_systems.push(hw_system);

    //Dynamic Lib for fast reload
    bevy_model.bevy_settings.features.push(Feature::Dynamic);

    bevy_model
}
