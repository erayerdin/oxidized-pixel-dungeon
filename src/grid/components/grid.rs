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

use crate::grid::constants::GRID_SIZE;

#[derive(Debug, Component, Reflect, Clone)]
/// Defines a single grid.
/// As `Cell` is a term used in Rust terminology, Grid is a better way to refer to this.
pub struct Grid {
    x: u8,
    y: u8,
}

impl Grid {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn transform(&self, z: f32) -> Transform {
        Transform::from_xyz(
            self.x as f32 * GRID_SIZE as f32,
            self.y as f32 * GRID_SIZE as f32,
            z,
        )
    }

    pub fn add_y_mut(&mut self) {
        self.y = self.y.checked_add(1).unwrap_or(self.y);
    }

    pub fn sub_y_mut(&mut self) {
        self.y = self.y.checked_sub(1).unwrap_or(self.y);
    }

    pub fn add_x_mut(&mut self) {
        self.x = self.x.checked_add(1).unwrap_or(self.x);
    }

    pub fn sub_x_mut(&mut self) {
        self.x = self.x.checked_sub(1).unwrap_or(self.x);
    }
}
