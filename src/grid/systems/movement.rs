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

type TransformGridQuery<'a> = (&'a mut Transform, &'a Grid);

pub(crate) fn character_movement_system(
    mut transform_grid_query: Query<TransformGridQuery, (With<Character>, Changed<Grid>)>,
) {
    trace_span!("Repositioning Character due to grid change...");

    for (mut transform, grid) in transform_grid_query.iter_mut() {
        transform.translation = grid.transform(0.0).translation;
    }
}
