// Copyright (C) 2026 Eray Erdin
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

use bevy::{platform::collections::Equivalent, prelude::*};
use bevy_pancam::PanCam;

use crate::screen::ScreenState;

pub fn camera_control_system(
    screen_state: Res<State<ScreenState>>,
    mut q1: Query<&mut PanCam, With<Camera2d>>,
) {
    debug!("Running camera_control_system...");
    if screen_state.equivalent(&ScreenState::Game) {
        debug!("Trying to enable camera controls...");
        match q1.single_mut() {
            Ok(mut pancam) => {
                debug!("Enabling camera controls...");
                pancam.enabled = true;
            }
            Err(err) => error!("{}", err),
        };
    } else {
        debug!("Disabling camera controls...");
        match q1.single_mut() {
            Ok(mut pancam) => {
                debug!("Disabling camera controls...");
                pancam.enabled = false;
            }
            Err(err) => error!("{}", err),
        };
    }
}
