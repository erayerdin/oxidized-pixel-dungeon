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

use crate::user_interface::{
    components::{DialogBox, Widget},
    UserInterfaceAssets,
};

#[derive(Debug, Component)]
pub struct DialogBox1;

#[derive(Debug, Builder)]
pub struct Dialogbox1WidgetProps<F>
where
    F: Fn(&mut ChildBuilder),
{
    #[builder(default = "Visibility::Hidden")]
    visibility: Visibility,
    #[builder(default = "1.0")]
    scale: f32,
    with_children: F,
}

pub fn dialogbox1_widget<F>(
    parent: &mut ChildBuilder,
    user_interface_assets: &Res<UserInterfaceAssets>,
    marker_component: impl Component,
    props: Dialogbox1WidgetProps<F>,
) where
    F: Fn(&mut ChildBuilder),
{
    let Dialogbox1WidgetProps {
        visibility,
        scale,
        with_children,
    } = props;

    parent
        .spawn((
            ImageBundle {
                visibility,
                image: user_interface_assets
                    .dialogbox1_asset_handle
                    .clone_weak()
                    .into(),
                ..default()
            },
            ImageScaleMode::Sliced(TextureSlicer {
                border: BorderRect::square(5.0),
                center_scale_mode: SliceScaleMode::Stretch,
                sides_scale_mode: SliceScaleMode::Stretch,
                max_corner_scale: scale,
            }),
            marker_component,
            DialogBox1,
            DialogBox,
            Widget,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(5.0 * scale)),
                            ..default()
                        },
                        ..default()
                    },
                    Widget,
                ))
                .with_children(with_children);
        });
}
