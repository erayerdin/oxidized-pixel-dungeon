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

use bevy::prelude::*;
use bevy_inspector_egui::quick::StateInspectorPlugin;

use crate::core::states::AppState;

use self::{
    resources::GameTime,
    states::GameplayState,
    systems::{disable_pancam_system, enable_pancam_system, walk_action_system},
};

pub mod resources;
pub mod states;
pub mod systems;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameplayState>()
            .register_type::<GameplayState>()
            .add_plugins(StateInspectorPlugin::<GameplayState>::default())
            .init_resource::<GameTime>()
            .register_type::<GameTime>()
            .add_systems(OnEnter(AppState::InGame), enable_pancam_system)
            .add_systems(OnExit(AppState::InGame), disable_pancam_system)
            .add_systems(Update, walk_action_system);
    }
}
