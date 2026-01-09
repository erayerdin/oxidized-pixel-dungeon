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
mod states;
mod systems;

use bevy::{platform::collections::Equivalent, prelude::*};

pub(crate) use states::ScreenState;

use crate::GameVariant;

pub(super) struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<states::ScreenState>().add_systems(
            Update,
            systems::loading_screen_system.run_if(in_state(states::ScreenState::Loading)),
        );

        if cfg!(debug_assertions) {
            app.add_systems(
                Startup,
                |variant: Res<GameVariant>, mut commands: Commands| {
                    if variant.equivalent(&GameVariant::ExampleTiles) {
                        commands.set_state(states::ScreenState::Game);
                    }
                },
            );
        }
    }
}
