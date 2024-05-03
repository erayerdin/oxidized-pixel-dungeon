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
    pub(crate) warrior_atlas: Handle<Image>,
    pub(crate) mage_atlas: Handle<Image>,
    pub(crate) rogue_atlas: Handle<Image>,
    pub(crate) huntress_atlas: Handle<Image>,
    pub(crate) duelist_atlas: Handle<Image>,
}
