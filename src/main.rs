use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pancam::PanCamPlugin;
use bevy_prototype_lyon::plugin::ShapePlugin;
use oxidized_pixel_dungeon::{core::CorePlugin, grid::GridPlugin};

fn main() {
    let mut app = App::new();

    let default_plugins = DefaultPlugins.set(if cfg!(debug_assertions) {
        LogPlugin {
            level: bevy::log::Level::TRACE,
            filter: "info,wgpu_core=warn,wgpu_hal=warn,oxidized_pixel_dungeon=trace".into(),
            ..Default::default()
        }
    } else {
        LogPlugin {
            level: bevy::log::Level::INFO,
            filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
            ..Default::default()
        }
    });

    app.add_plugins((
        default_plugins.set(ImagePlugin::default_nearest()),
        PanCamPlugin,
        ShapePlugin,
    ))
    .add_plugins((CorePlugin, GridPlugin));

    if cfg!(debug_assertions) {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}
