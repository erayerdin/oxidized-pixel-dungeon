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

#[derive(Debug, Clone)]
pub enum Icon {
    GoldCoins,
}

impl Icon {
    fn ui_image(&self, user_interface_assets: &Res<UserInterfaceAssets>) -> UiImage {
        match self {
            Icon::GoldCoins => user_interface_assets
                .gold_coin_icon_handle
                .clone_weak()
                .into(),
        }
    }
}

#[derive(Debug, Clone, Builder)]
pub struct IconWidgetProps {
    icon: Icon,
    #[builder(default = "16.0")]
    size: f32,
}

pub fn icon_widget(
    parent: &mut ChildBuilder,
    user_interface_assets: &Res<UserInterfaceAssets>,
    props: IconWidgetProps,
) {
    let IconWidgetProps { icon, size } = props;

    let ui_image = icon.ui_image(user_interface_assets);

    parent.spawn((
        ImageBundle {
            image: ui_image,
            style: Style {
                width: Val::Px(size),
                height: Val::Px(size),
                ..default()
            },
            ..default()
        },
        Name::new("Icon"),
        Widget,
    ));
}
