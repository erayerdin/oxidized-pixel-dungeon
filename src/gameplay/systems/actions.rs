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
    core::{components::FacingDirection, constants::CHARACTER_Z_INDEX},
    gameplay::{resources::GameTime, states::GameplayState},
    grid::components::grid::Grid,
    mob::components::Hero,
};

pub(crate) fn walk_action_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut gameplay_state: ResMut<NextState<GameplayState>>,
    mut game_time: ResMut<GameTime>,
    mut query: Query<(&mut Transform, &mut Grid, &mut FacingDirection, &mut Sprite), With<Hero>>,
) {
    gameplay_state.set(GameplayState::Transitioning);

    let (mut transform, mut grid, mut facing_direction, mut sprite) = query.single_mut();

    if keys.just_pressed(KeyCode::ArrowUp) {
        grid.add_y_mut();
        *transform = grid.transform(CHARACTER_Z_INDEX);
        game_time.counter += GameTime::base_walk_time();
    }

    if keys.just_pressed(KeyCode::ArrowDown) {
        grid.sub_y_mut();
        *transform = grid.transform(CHARACTER_Z_INDEX);
        game_time.counter += GameTime::base_walk_time();
    }

    if keys.just_pressed(KeyCode::ArrowLeft) {
        grid.sub_x_mut();
        if *facing_direction != FacingDirection::West {
            *facing_direction = FacingDirection::West;
            sprite.flip_x = true;
        }
        *transform = grid.transform(CHARACTER_Z_INDEX);
        game_time.counter += GameTime::base_walk_time();
    }

    if keys.just_pressed(KeyCode::ArrowRight) {
        grid.add_x_mut();
        if *facing_direction != FacingDirection::East {
            *facing_direction = FacingDirection::East;
            sprite.flip_x = false;
        }
        *transform = grid.transform(CHARACTER_Z_INDEX);
        game_time.counter += GameTime::base_walk_time();
    }

    gameplay_state.set(GameplayState::Awaiting);
}
