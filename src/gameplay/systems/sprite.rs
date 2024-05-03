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

use crate::{
    core::components::Character,
    // gameplay::states::GameplayState,
    grid::{components::grid::Grid, constants::GRID_SIZE},
};

pub(crate) fn sprite_move_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Grid), With<Character>>,
    // mut gameplay_state: ResMut<NextState<GameplayState>>,
) {
    for (mut transform, grid) in query.iter_mut() {
        let grid_transform = grid.transform(2.0);
        let grid_vec3 = grid_transform.translation;
        let transform_vec3 = transform.translation;

        if transform_vec3.x < grid_vec3.x {
            transform.translation.x += (GRID_SIZE as f32 * time.delta_seconds()) * 8.0;
        }
        if transform_vec3.x > grid_vec3.x {
            transform.translation.x -= (GRID_SIZE as f32 * time.delta_seconds()) * 8.0;
        }
        if transform_vec3.y < grid_vec3.y {
            transform.translation.y += (GRID_SIZE as f32 * time.delta_seconds()) * 8.0;
        }
        if transform_vec3.y > grid_vec3.y {
            transform.translation.y -= (GRID_SIZE as f32 * time.delta_seconds()) * 8.0;
        }

        // let difference = [
        //     transform_vec3.x - grid_vec3.x,
        //     transform_vec3.y - grid_vec3.y,
        // ]
        // .iter()
        // .sum::<f32>()
        // .abs();

        // if difference < 0.02 {
        //     transform.translation = grid_vec3;
        //     gameplay_state.set(GameplayState::Awaiting);
        // }
    }
}
