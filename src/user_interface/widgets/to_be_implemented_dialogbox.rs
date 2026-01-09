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

use crate::user_interface::{components::Overlay, UserInterfaceAssets};

use super::primitives::{dialogbox1_widget, Dialogbox1WidgetPropsBuilder};

#[derive(Debug, Component)]
pub struct ToBeImplementedDialogbox;

pub fn to_be_implemented_dialogbox(
    commands: &mut Commands,
    user_interface_assets: &Res<UserInterfaceAssets>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            Overlay,
            Interaction::None,
        ))
        .with_children(|parent| {
            dialogbox1_widget(
                parent,
                user_interface_assets,
                ToBeImplementedDialogbox,
                Dialogbox1WidgetPropsBuilder::default()
                    .scale(2.0)
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Not implemented yet",
                            TextStyle {
                                font: user_interface_assets.pixel_font_asset_handle.clone_weak(),
                                font_size: 16.0,
                                color: Color::WHITE,
                            },
                        ));
                    })
                    .build()
                    .unwrap(),
            );
        });
}
