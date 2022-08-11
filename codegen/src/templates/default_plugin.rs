use crate::{
    model::{Component, Meta, Plugin, System},
    BevyModel, BevyType,
};

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
        visibility: "pub".to_string(),
        attributes: vec![],
    };
    bevy_model.startup_systems.push(hw_system);

    bevy_model
}
