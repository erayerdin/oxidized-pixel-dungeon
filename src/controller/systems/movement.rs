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

use crate::{core::components::Character, grid::components::grid::Grid};

pub(crate) fn character_movement_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut transform_grid_player_query: Query<(&mut Transform, &mut Grid), With<Character>>,
) {
    let (mut transform, mut grid) = transform_grid_player_query.single_mut();
    let mut new_grid = grid.clone();

    if keys.pressed(KeyCode::ArrowLeft) {
        new_grid = grid.sub_x();
        transform.translation = new_grid.transform(0.0).translation;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        new_grid = grid.add_x();
        transform.translation = new_grid.transform(0.0).translation;
    }
    if keys.pressed(KeyCode::ArrowUp) {
        new_grid = grid.add_y();
        transform.translation = new_grid.transform(0.0).translation;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        new_grid = grid.sub_y();
        transform.translation = new_grid.transform(0.0).translation;
    }

    *grid = new_grid;
}

pub(crate) fn character_sprite_flip_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut sprite_query: Query<&mut Sprite, With<Character>>,
) {
    let mut sprite = sprite_query.single_mut();
    let flip_x = keys.just_pressed(KeyCode::ArrowLeft);

    sprite.flip_x = flip_x;
}
