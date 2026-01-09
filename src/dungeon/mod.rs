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

mod components;
mod constants;
mod resources;
mod systems;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::screen::ScreenState;

pub(super) struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::Floor>()
            .add_loading_state(
                LoadingState::new(ScreenState::Loading)
                    .continue_to_state(ScreenState::Game)
                    .load_collection::<resources::DungeonAssets>(),
            )
            .add_systems(Startup, systems::dungeon_init_system)
            .add_systems(Update, systems::dungeon_render_system);
    }
}
