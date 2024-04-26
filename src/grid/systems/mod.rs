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

use bevy::{prelude::*, sprite::*};
use bevy_prototype_lyon::prelude::*;

use crate::grid::{components::grid::Grid, constants::GRID_SIZE};

pub(crate) fn init_grids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    debug!("Initializing grids...");

    for x in 0..u8::MAX {
        for y in 0..u8::MAX {
            let grid = Grid::new(x, y);

            // let mesh_handler =
            //     Mesh2dHandle(meshes.add(Rectangle::new(GRID_SIZE as f32, GRID_SIZE as f32)));

            // let coordination_sum = x as u16 + y as u16;
            // let color = if coordination_sum % 2 == 0 {
            //     Color::WHITE
            // } else {
            //     Color::BLACK
            // };

            let (pos_x, pos_y) = grid.positions();

            let shape = shapes::Rectangle {
                extents: Vec2 {
                    x: GRID_SIZE as f32,
                    y: GRID_SIZE as f32,
                },
                ..default()
            };

            commands.spawn((
                grid,
                // MaterialMesh2dBundle {
                //     mesh: mesh_handler,
                //     material: materials.add(color),
                //     transform: Transform::from_xyz(pos_x, pos_y, 0.0),
                //     ..Default::default()
                // },
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(pos_x, pos_y, 0.0),
                        ..default()
                    },
                    ..default()
                },
                Stroke::new(Color::WHITE, 1.0),
            ));
        }
    }
}
