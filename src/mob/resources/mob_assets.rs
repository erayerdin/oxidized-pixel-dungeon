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
use bevy_asset_loader::prelude::*;

use crate::{
    core::constants::CHARACTER_Z_INDEX, grid::components::grid::Grid, mob::components::HeroClass,
};

#[derive(Debug, Resource, AssetCollection)]
pub(crate) struct HeroAssets {
    #[asset(path = "spd/sprites/warrior.png")]
    warrior_image_handle: Handle<Image>,
    #[asset(path = "spd/sprites/mage.png")]
    mage_image_handle: Handle<Image>,
    #[asset(path = "spd/sprites/rogue.png")]
    rogue_image_handle: Handle<Image>,
    #[asset(path = "spd/sprites/huntress.png")]
    huntress_image_handle: Handle<Image>,
    #[asset(path = "spd/sprites/duelist.png")]
    duelist_image_handle: Handle<Image>,
}

impl HeroAssets {
    pub(crate) fn layout(&self) -> TextureAtlasLayout {
        TextureAtlasLayout::from_grid(
            Vec2::new(11.0, 15.0),
            21,
            7,
            Some(Vec2::new(1.0, 0.0)),
            None,
        )
    }

    pub(crate) fn layout_handle(
        &self,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> Handle<TextureAtlasLayout> {
        texture_atlas_layouts.add(self.layout())
    }

    pub(crate) fn image_handle(&self, hero_class: &HeroClass) -> Handle<Image> {
        match hero_class {
            HeroClass::Warrior => self.warrior_image_handle.clone_weak(),
            HeroClass::Mage => self.mage_image_handle.clone_weak(),
            HeroClass::Rogue => self.rogue_image_handle.clone_weak(),
            HeroClass::Huntress => self.huntress_image_handle.clone_weak(),
            HeroClass::Duelist => self.duelist_image_handle.clone_weak(),
        }
    }

    pub(crate) fn sprite_sheet_bundle(
        &self,
        hero_class: &HeroClass,
        grid: &Grid,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
        texture_atlas_index: usize,
    ) -> SpriteSheetBundle {
        SpriteSheetBundle {
            texture: self.image_handle(hero_class),
            atlas: TextureAtlas {
                layout: self.layout_handle(texture_atlas_layouts),
                index: texture_atlas_index,
            },
            transform: grid.transform(CHARACTER_Z_INDEX),
            ..default()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub(crate) enum HeroAssetsLoadState {
    #[default]
    Loading,
    Loaded,
}
