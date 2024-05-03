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

use bevy::{asset::AssetPath, prelude::*};

#[derive(Debug, Component)]
pub(crate) struct Hero;

#[derive(Debug, Copy, Clone, Component, Reflect)]
pub(crate) enum HeroClass {
    Warrior,
    Mage,
    Rogue,
    Huntress,
    Duelist,
}

impl<'a> From<&HeroClass> for AssetPath<'a> {
    fn from(value: &HeroClass) -> Self {
        match value {
            HeroClass::Warrior => "spd/sprites/warrior.png".into(),
            HeroClass::Mage => "spd/sprites/mage.png".into(),
            HeroClass::Rogue => "spd/sprites/rogue.png".into(),
            HeroClass::Huntress => "spd/sprites/huntress.png".into(),
            HeroClass::Duelist => "spd/sprites/duelist.png".into(),
        }
    }
}
