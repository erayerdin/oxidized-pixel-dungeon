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
    Downstairs,
}

#[derive(Debug, Builder)]
pub struct IconWidgetProps {
    icon: Icon,
    #[builder(default = "1.0")]
    scale: f32,
}

pub fn icon_widget(
    parent: &mut ChildBuilder,
    user_interface_assets: &Res<UserInterfaceAssets>,
    props: IconWidgetProps,
) {
    let IconWidgetProps { icon, scale } = props;

    parent.spawn((
        ImageBundle {
            image: UiImage::from(user_interface_assets.icons_asset_handle.clone_weak()),
            ..default()
        },
        Widget,
    ));
}
