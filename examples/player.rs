// Copyright (C) 2024 Eray Erdin
//
// This file is part of Oxidized Pixel Dungeon.
//
// Oxidized Pixel Dungeon is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Oxidized Pixel Dungeon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Oxidized Pixel Dungeon.  If not, see <https://www.gnu.org/licenses/>.

use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pancam::PanCamPlugin;
use bevy_prototype_lyon::plugin::ShapePlugin;
use oxidized_pixel_dungeon::{
    core::CorePlugin, gameplay::GameplayPlugin, grid::GridPlugin, player::PlayerPlugin,
};

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
    .add_plugins((CorePlugin, GridPlugin, PlayerPlugin, GameplayPlugin));

    if cfg!(debug_assertions) {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}
