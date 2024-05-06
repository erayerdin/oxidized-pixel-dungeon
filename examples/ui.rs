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
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use oxidized_pixel_dungeon::{
    core::CorePlugin,
    user_interface::{UserInterfaceAssetsLoadState, UserInterfacePlugin},
};

fn main() {
    let mut app = App::new();

    app.add_plugins((CorePlugin, UserInterfacePlugin))
        .add_systems(
            OnEnter(UserInterfaceAssetsLoadState::LoadedState),
            systems::layout_init_system,
        );

    if cfg!(debug_assertions) {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}

mod systems {
    use bevy::prelude::*;
    use oxidized_pixel_dungeon::user_interface::{
        widgets::{
            button1_widget, icon_widget, Button1WidgetPropsBuilder, Icon, IconWidgetPropsBuilder,
        },
        UserInterfaceAssets,
    };

    pub fn layout_init_system(
        mut commands: Commands,
        user_interface_assets: Res<UserInterfaceAssets>,
    ) {
        commands
            .spawn(NodeBundle {
                background_color: Color::RED.into(),
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                button1_widget(
                    parent,
                    &user_interface_assets,
                    Button1WidgetPropsBuilder::default()
                        .text("Normal Text")
                        .build()
                        .unwrap(),
                );
                button1_widget(
                    parent,
                    &user_interface_assets,
                    Button1WidgetPropsBuilder::default()
                        .text("Yellow text")
                        .font_color(Color::YELLOW)
                        .build()
                        .unwrap(),
                );
                button1_widget(
                    parent,
                    &user_interface_assets,
                    Button1WidgetPropsBuilder::default()
                        .text("Bigger font")
                        .font_size(20.0)
                        .build()
                        .unwrap(),
                );
                icon_widget(
                    parent,
                    &user_interface_assets,
                    IconWidgetPropsBuilder::default()
                        .icon(Icon::GoldCoins)
                        .build()
                        .unwrap(),
                );
                parent.spawn(TextBundle::from_section(
                    "Icon 1x",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
    }
}
