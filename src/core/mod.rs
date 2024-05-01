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
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_prototype_lyon::prelude::*;

use crate::{grid::GridPlugin, player::PlayerPlugin};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
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
        .add_systems(Startup, init_system)
        .add_plugins((GridPlugin, PlayerPlugin));
    }
}

fn init_system(mut commands: Commands) {
    debug!("Running CorePlugin::init_system");
    commands.spawn(Camera2dBundle::default()).insert(PanCam {
        grab_buttons: vec![MouseButton::Middle],
        enabled: true,
        ..default()
    });
}
