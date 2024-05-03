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
    mob::{
        components::{Hero, HeroClass},
        resources::HeroAssets,
    },
};

pub(crate) fn player_init_system(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    hero_assets: Res<HeroAssets>,
) {
    debug!("Initializing player...");

    let hero_class = HeroClass::Warrior;

    let grid = Grid::new(0, 0);

    commands.spawn((
        Name::new("Player"),
        Character,
        Hero,
        hero_class,
        FacingDirection::default(),
        hero_assets.sprite_sheet_bundle(&hero_class, &grid, &mut texture_atlas_layouts, 0),
        grid,
    ));
}

#[cfg(debug_assertions)]
pub(crate) fn player_texture_change_system(
    mut query: Query<(&mut Handle<Image>, &HeroClass), (With<Hero>, Changed<HeroClass>)>,
    hero_assets: Option<Res<HeroAssets>>,
) {
    if let Some(hero_assets) = hero_assets {
        for (mut handle, hero_class) in query.iter_mut() {
            *handle = hero_assets.image_handle(hero_class);
        }
    }
}
