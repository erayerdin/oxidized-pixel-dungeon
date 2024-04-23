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

use bevy::{
    asset::Assets,
    ecs::system::{Commands, Query, ResMut},
    log::debug,
    math::primitives::Rectangle,
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
    transform::components::Transform,
};

use crate::grid::{components::grid::Grid, constants::GRID_SIZE};

pub(crate) fn init_grids(mut commands: Commands) {
    debug!("Initializing grids...");

    for x in 0..u8::MAX {
        for y in 0..u8::MAX {
            commands.spawn(Grid::new(x, y));
        }
    }
}

pub(crate) fn init_meshes(
    grid_query: Query<&Grid>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // REF: https://bevyengine.org/examples/2D%20Rendering/2d-shapes/
    debug!("Painting grids...");

    for grid in grid_query.iter() {
        // let color = if (grid_x + grid_y) % 2 == 0 {
        //     Color::BLACK
        // } else {
        //     Color::WHITE
        // };

        let (pos_x, pos_y) = grid.positions();

        let mesh_handler =
            Mesh2dHandle(meshes.add(Rectangle::new(GRID_SIZE as f32, GRID_SIZE as f32)));

        commands.spawn(MaterialMesh2dBundle {
            mesh: mesh_handler,
            material: materials.add(Color::BLACK),
            transform: Transform::from_xyz(pos_x as f32, pos_y as f32, 0.0),
            ..Default::default()
        });
    }
}
