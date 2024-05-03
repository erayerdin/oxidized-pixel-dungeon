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

use crate::{grid::components::grid::Grid, player::components::Player};

pub(crate) fn move_action_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut grid_query: Query<&mut Grid, With<Player>>,
) {
    for mut grid in grid_query.iter_mut() {
        if keys.just_pressed(KeyCode::ArrowUp) {
            grid.add_y_mut();
        }
        if keys.just_pressed(KeyCode::ArrowDown) {
            grid.sub_y_mut();
        }
        if keys.just_pressed(KeyCode::ArrowLeft) {
            grid.sub_x_mut();
        }
        if keys.just_pressed(KeyCode::ArrowRight) {
            grid.add_x_mut();
        }
    }
}
