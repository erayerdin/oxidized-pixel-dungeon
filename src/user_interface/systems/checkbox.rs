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

use crate::user_interface::{components::Widget, widgets::Checkbox, UserInterfaceAssets};

pub fn checkbox_check_system(
    mut query: Query<
        (&Interaction, &mut UiImage, &mut Checkbox),
        (With<Widget>, Changed<Interaction>),
    >,
    user_interface_assets: Option<Res<UserInterfaceAssets>>,
) {
    if let Some(user_interface_assets) = user_interface_assets {
        for (interaction, mut ui_image, mut checkbox) in query.iter_mut() {
            if interaction == &Interaction::Pressed {
                checkbox.0 = !checkbox.0;
                let is_checked = checkbox.0;
                *ui_image = if is_checked {
                    UiImage::from(
                        user_interface_assets
                            .checkbox_checked_asset_handle
                            .clone_weak(),
                    )
                } else {
                    UiImage::from(
                        user_interface_assets
                            .checkbox_unchecked_asset_handle
                            .clone_weak(),
                    )
                }
            }
        }
    }
}
