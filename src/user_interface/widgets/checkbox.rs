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
use derive_builder::Builder;

use crate::user_interface::{components::Widget, UserInterfaceAssets};

#[derive(Debug, Builder)]
pub struct CheckboxWidgetProps {
    #[builder(setter(custom))]
    text: String,
    #[builder(default = "false")]
    checked: bool,
    #[builder(default = "16.0")]
    size: f32,
}

impl CheckboxWidgetPropsBuilder {
    pub fn text(&mut self, text: impl Into<String>) -> &mut Self {
        self.text = Some(text.into());
        self
    }
}

pub fn checkbox_widget(
    parent: &mut ChildBuilder,
    user_interface_assets: &Res<UserInterfaceAssets>,
    props: CheckboxWidgetProps,
) {
    let CheckboxWidgetProps {
        text,
        checked,
        size,
    } = props;

    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Auto,
                    height: Val::Auto,
                    ..default()
                },
                ..default()
            },
            Widget,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    image: UiImage::from(if checked {
                        user_interface_assets
                            .checkbox_checked_asset_handle
                            .clone_weak()
                    } else {
                        user_interface_assets
                            .checkbox_unchecked_asset_handle
                            .clone_weak()
                    }),
                    style: Style {
                        width: Val::Px(size),
                        height: Val::Px(size),
                        ..default()
                    },
                    ..default()
                },
                Widget,
            ));

            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        text,
                        TextStyle {
                            font: user_interface_assets.pixel_font_asset_handle.clone_weak(),
                            font_size: size,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                },
                Widget,
            ));
        });
}
