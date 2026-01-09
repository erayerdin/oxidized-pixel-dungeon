// Copyright (C) 2026 Eray Erdin
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

mod camera;
mod dungeon;
mod screen;

use bevy::{
    log::{self, LogPlugin},
    prelude::*,
};

#[derive(Default, Resource, Clone, Eq, PartialEq)]
pub enum GameVariant {
    #[default]
    Regular,
    #[cfg(debug_assertions)]
    ExampleLoading,
    #[cfg(debug_assertions)]
    ExampleTiles,
}

#[derive(Default)]
pub struct GamePlugin {
    variant: GameVariant,
}

impl GamePlugin {
    pub fn new(variant: GameVariant) -> Self {
        Self { variant }
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(self.variant.clone())
            // Built-in plugins
            .add_plugins(DefaultPlugins.set(LogPlugin {
                level: if cfg!(debug_assertions) {
                    log::Level::DEBUG
                } else {
                    log::Level::WARN
                },
                filter:
                    "wgpu=warn,bevy_ecs=info,bevy_shader=info,bevy_time=info,bevy_render=info,bevy_asset=info,bevy_winit=info,bevy_app=info,gilrs=info,cosmic_text=info,winit=info,sctk=info,offset_allocator=info,naga=info,calloop=info".to_string(),
                ..default()
            }));

        match self.variant {
            GameVariant::Regular => {
                // project plugins
                app.add_plugins((
                    camera::CameraPlugin,
                    screen::ScreenPlugin,
                    dungeon::DungeonPlugin,
                ));
            }
            #[cfg(debug_assertions)]
            GameVariant::ExampleLoading => {
                // project plugins
                app.add_plugins((camera::CameraPlugin, screen::ScreenPlugin));
            }
            #[cfg(debug_assertions)]
            GameVariant::ExampleTiles => {
                app.add_plugins((camera::CameraPlugin, dungeon::DungeonPlugin));
            }
        }
    }
}
