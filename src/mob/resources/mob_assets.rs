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

#[derive(Debug, Resource)]
pub(crate) struct HeroAssets {
    warrior_image_handle: Handle<Image>,
    mage_image_handle: Handle<Image>,
    rogue_image_handle: Handle<Image>,
    huntress_image_handle: Handle<Image>,
    duelist_image_handle: Handle<Image>,
}

impl HeroAssets {
    pub(crate) fn new(
        warrior_image_handle: Handle<Image>,
        mage_image_handle: Handle<Image>,
        rogue_image_handle: Handle<Image>,
        huntress_image_handle: Handle<Image>,
        duelist_image_handle: Handle<Image>,
    ) -> Self {
        Self {
            warrior_image_handle,
            mage_image_handle,
            rogue_image_handle,
            huntress_image_handle,
            duelist_image_handle,
        }
    }
}