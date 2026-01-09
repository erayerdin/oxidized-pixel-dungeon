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

use bevy::prelude::*;
use bevy_pancam::{DirectionKeys, PanCam};

pub(super) fn default_pancam() -> PanCam {
    PanCam {
        grab_buttons: vec![MouseButton::Middle],
        move_keys: DirectionKeys {
            up: vec![KeyCode::ArrowUp],
            down: vec![KeyCode::ArrowDown],
            left: vec![KeyCode::ArrowLeft],
            right: vec![KeyCode::ArrowRight],
        },
        speed: 400.0,
        enabled: true,
        zoom_to_cursor: false,
        min_scale: 1.0,
        max_scale: 40.0,
        min_x: f32::NEG_INFINITY, // minimum x position of the camera window
        max_x: f32::INFINITY,     // maximum x position of the camera window
        min_y: f32::NEG_INFINITY, // minimum y position of the camera window
        max_y: f32::INFINITY,     // maximum y position of the camera window
        mouse_wheel_sensitivity: 0.5,
        pinch_gesture_sensitivity: 0.5,
    }
}
