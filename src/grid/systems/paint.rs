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

use crate::{core::resources::game_config::GameConfig, grid::components::grid::Grid};

pub(crate) fn grid_paint_system(
    mut visibility_query: Query<&mut Visibility, With<Grid>>,
    game_config: Res<GameConfig>,
) {
    debug!("Painting grids...");

    // REF: https://github.com/bevyengine/bevy/blob/64b987921c4a4d54c3f250ae63bb9eb6b44a02aa/examples/ecs/parallel_query.rs#L31
    visibility_query.par_iter_mut().for_each(|mut visibility| {
        *visibility = game_config.grid_visibility();
    });
}
