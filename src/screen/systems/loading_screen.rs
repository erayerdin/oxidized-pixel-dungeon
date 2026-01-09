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

use crate::screen::components::LoadingUI;

pub fn loading_screen_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // REF: https://bevy.org/examples/ui-user-interface/flex-layout/
    let font = asset_server.load("opd/fonts/Jersey10-Regular.ttf");

    commands
        .spawn(LoadingUI)
        .insert((
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::BLACK),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text::new("Loading..."),
                TextFont {
                    font,
                    font_size: 24.0,
                    ..default()
                },
                TextColor::WHITE,
            ));
        });
}
