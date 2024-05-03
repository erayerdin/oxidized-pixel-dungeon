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

/// In-game time that defines the turns.
#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
pub(crate) struct GameTime {
    pub(crate) counter: f32,
}

impl GameTime {
    pub(crate) const fn base_walk_time() -> f32 {
        1.0
    }
}

impl Default for GameTime {
    fn default() -> Self {
        Self { counter: 0.0 }
    }
}
