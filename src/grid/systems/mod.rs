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
use bevy_prototype_lyon::prelude::*;

use crate::{
    core::resources::game_config::GameConfig,
    grid::{components::grid::Grid, constants::GRID_SIZE},
};

pub(crate) fn grid_init_system(game_config: Res<GameConfig>, mut commands: Commands) {
    debug!("Initializing grids...");

    for x in 0..u8::MAX {
        for y in 0..u8::MAX {
            let grid = Grid::new(x, y);

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
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(pos_x, pos_y, 0.0),
                        visibility: game_config.grid_visibility(),
                        ..default()
                    },
                    ..default()
                },
                Stroke::new(Color::WHITE, 1.0),
            ));
        }
    }
}

pub(crate) fn grid_paint_system(
    mut visibility_query: Query<&mut Visibility, With<Grid>>,
    game_config: Res<GameConfig>,
) {
    debug!("Painting grids...");

    visibility_query.par_iter_mut().for_each(|mut visibility| {
        *visibility = game_config.grid_visibility();
    });
}
