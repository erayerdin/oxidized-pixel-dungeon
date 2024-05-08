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
use systems::init_system;

fn main() {
    let mut app = App::new();

    app.add_plugins((CorePlugin, UserInterfacePlugin))
        .add_systems(
            OnEnter(UserInterfaceAssetsLoadState::LoadedState),
            init_system,
        );

    if cfg!(debug_assertions) {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}

mod components {
    use bevy::prelude::*;

    #[derive(Debug, Component)]
    pub struct StartTheGameButton;

    #[derive(Debug, Component)]
    pub struct SupportTheGameButton;

    #[derive(Debug, Component)]
    pub struct RankingButton;

    #[derive(Debug, Component)]
    pub struct NewsButton;

    #[derive(Debug, Component)]
    pub struct SettingsButton;

    #[derive(Debug, Component)]
    pub struct BadgesButton;

    #[derive(Debug, Component)]
    pub struct ChangesButton;

    #[derive(Debug, Component)]
    pub struct AboutButton;
}

mod systems {
    use bevy::prelude::*;
    use oxidized_pixel_dungeon::user_interface::{
        widgets::{button1_widget, Button1WidgetPropsBuilder, Icon, IconWidgetPropsBuilder},
        UserInterfaceAssets,
    };

    use crate::components::{
        AboutButton, BadgesButton, ChangesButton, NewsButton, RankingButton, SettingsButton,
        StartTheGameButton, SupportTheGameButton,
    };

    pub fn init_system(mut commands: Commands, user_interface_assets: Res<UserInterfaceAssets>) {
        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            max_width: Val::Percent(60.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(8.0),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        // first row of buttons
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    column_gap: Val::Px(8.0),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                button1_widget(
                                    parent,
                                    &user_interface_assets,
                                    StartTheGameButton,
                                    Button1WidgetPropsBuilder::default()
                                        .text("Start the Game")
                                        .icon_widget_props(Some(
                                            IconWidgetPropsBuilder::default()
                                                .icon(Icon::Downstairs)
                                                .build()
                                                .unwrap(),
                                        ))
                                        .build()
                                        .unwrap(),
                                );
                                button1_widget(
                                    parent,
                                    &user_interface_assets,
                                    SupportTheGameButton,
                                    Button1WidgetPropsBuilder::default()
                                        .text("Support the Game")
                                        .icon_widget_props(Some(
                                            IconWidgetPropsBuilder::default()
                                                .icon(Icon::GoldCoins)
                                                .build()
                                                .unwrap(),
                                        ))
                                        .font_color(Color::YELLOW)
                                        .build()
                                        .unwrap(),
                                );
                            });

                        // second row of buttons
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    column_gap: Val::Px(8.0),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                button1_widget(
                                    parent,
                                    &user_interface_assets,
                                    RankingButton,
                                    Button1WidgetPropsBuilder::default()
                                        .text("Ranking")
                                        .icon_widget_props(Some(
                                            IconWidgetPropsBuilder::default()
                                                .icon(Icon::RankingLadders)
                                                .build()
                                                .unwrap(),
                                        ))
                                        .build()
                                        .unwrap(),
                                );
                                button1_widget(
                                    parent,
                                    &user_interface_assets,
                                    NewsButton,
                                    Button1WidgetPropsBuilder::default()
                                        .text("News")
                                        .icon_widget_props(Some(
                                            IconWidgetPropsBuilder::default()
                                                .icon(Icon::FrontFacingScroll)
                                                .build()
                                                .unwrap(),
                                        ))
                                        .build()
                                        .unwrap(),
                                );
                                button1_widget(
                                    parent,
                                    &user_interface_assets,
                                    SettingsButton,
                                    Button1WidgetPropsBuilder::default()
                                        .text("Settings")
                                        .icon_widget_props(Some(
                                            IconWidgetPropsBuilder::default()
                                                .icon(Icon::Cogs)
                                                .build()
                                                .unwrap(),
                                        ))
                                        .build()
                                        .unwrap(),
                                );
                            });

                        // third row of buttons
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    column_gap: Val::Px(8.0),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                button1_widget(
                                    parent,
                                    &user_interface_assets,
                                    BadgesButton,
                                    Button1WidgetPropsBuilder::default()
                                        .text("Badges")
                                        .icon_widget_props(Some(
                                            IconWidgetPropsBuilder::default()
                                                .icon(Icon::CrestOfBlade)
                                                .build()
                                                .unwrap(),
                                        ))
                                        .build()
                                        .unwrap(),
                                );
                                button1_widget(
                                    parent,
                                    &user_interface_assets,
                                    ChangesButton,
                                    Button1WidgetPropsBuilder::default()
                                        .text("Changes")
                                        .icon_widget_props(Some(
                                            IconWidgetPropsBuilder::default()
                                                .icon(Icon::OuroborosArrows)
                                                .build()
                                                .unwrap(),
                                        ))
                                        .build()
                                        .unwrap(),
                                );
                                button1_widget(
                                    parent,
                                    &user_interface_assets,
                                    AboutButton,
                                    Button1WidgetPropsBuilder::default()
                                        .text("About")
                                        .icon_widget_props(Some(
                                            IconWidgetPropsBuilder::default()
                                                .icon(Icon::ShatteredBlock)
                                                .build()
                                                .unwrap(),
                                        ))
                                        .build()
                                        .unwrap(),
                                );
                            });
                    });
            });
    }
}
