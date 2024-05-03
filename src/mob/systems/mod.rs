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
    core::components::{Character, FacingDirection},
    grid::components::grid::Grid,
    mob::components::{Player, PlayerClass},
};

pub(crate) fn player_init_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    debug!("Initializing player...");

    let player_class = PlayerClass::Warrior;

    let texture: Handle<Image> = asset_server.load(&player_class);
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(11.0, 15.0),
        21,
        7,
        Some(Vec2::new(1.0, 0.0)),
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let grid = Grid::new(0, 0);

    commands.spawn((
        Name::new("Player"),
        Character,
        Player,
        player_class,
        FacingDirection::default(),
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            transform: grid.transform(0.0),
            ..default()
        },
        grid,
    ));
}
