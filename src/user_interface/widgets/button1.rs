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

use crate::user_interface::{components::Widget, UserInterfaceAssets};

pub fn button1_widget(
    parent: &mut ChildBuilder,
    user_interface_assets: &Res<UserInterfaceAssets>,
    text: impl Into<String>,
) {
    parent
        .spawn(NodeBundle {
            background_color: Color::BLACK.with_a(0.5).into(),
            style: Style {
                width: Val::Auto,
                height: Val::Auto,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Auto,
                            height: Val::Auto,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(8.0)),
                            ..default()
                        },
                        image: user_interface_assets
                            .button1_asset_handle
                            .clone_weak()
                            .into(),
                        ..default()
                    },
                    ImageScaleMode::Sliced(TextureSlicer {
                        border: BorderRect::square(2.0),
                        center_scale_mode: SliceScaleMode::Stretch,
                        sides_scale_mode: SliceScaleMode::Stretch,
                        max_corner_scale: 1.0,
                    }),
                    Widget,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            text,
                            TextStyle {
                                font: user_interface_assets.pixel_font_asset_handle.clone_weak(),
                                font_size: 36.0,
                                color: Color::WHITE,
                            },
                        ),
                        Widget,
                    ));
                });
        });
}
