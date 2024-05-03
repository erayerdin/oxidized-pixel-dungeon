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

use self::{
    states::GameplayState,
    systems::{
        gameplay_state_transitioning_setter_system, move_action_system,
        sprite_facing_direction_system, sprite_move_system,
    },
};

pub(crate) mod constants;
pub(crate) mod states;
pub(crate) mod systems;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameplayState>()
            .register_type::<GameplayState>()
            .add_plugins(StateInspectorPlugin::<GameplayState>::default())
            .add_systems(
                Update,
                (
                    gameplay_state_transitioning_setter_system
                        .run_if(in_state(GameplayState::Awaiting)),
                    move_action_system.run_if(in_state(GameplayState::Awaiting)),
                    sprite_move_system.run_if(in_state(GameplayState::Transitioning)),
                    sprite_facing_direction_system,
                ),
            );
    }
}
