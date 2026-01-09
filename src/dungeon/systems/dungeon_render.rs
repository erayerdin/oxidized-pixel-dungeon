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

use bevy::{platform::collections::Equivalent, prelude::*};

use crate::{
    GameVariant,
    dungeon::{
        components::Grid,
        constants::{GRID_SIZE, GridDebugColor},
    },
};

pub fn dungeon_render_system(
    q1: Query<(Entity, &Grid)>,
    variant: Res<GameVariant>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    // TODO: parallelize
    for (entity, grid) in q1.iter() {
        let (x, y) = grid.xy();

        commands.entity(entity).insert(Transform::from_xyz(
            (x * GRID_SIZE as u16) as f32,
            (y * GRID_SIZE as u16) as f32,
            // BUG: this might cause z-fighting
            0.0,
        ));

        if variant.equivalent(&GameVariant::ExampleTiles) {
            commands.entity(entity).insert((
                Mesh2d(meshes.add(Rectangle::new(GRID_SIZE as f32, GRID_SIZE as f32))),
                MeshMaterial2d(materials.add(GridDebugColor::Ground)),
            ));
        }
    }
}
