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
    core::{states::AppState, CorePlugin},
    user_interface::{UserInterfaceAssetsLoadState, UserInterfacePlugin},
};
use resources::Counter;

fn main() {
    let mut app = App::new();

    app.add_plugins((CorePlugin, UserInterfacePlugin))
        .init_resource::<Counter>()
        .add_systems(
            OnEnter(UserInterfaceAssetsLoadState::LoadedState),
            systems::layout_init_system,
        )
        .add_systems(Update, systems::counter_system);

    app.insert_state(AppState::None);

    if cfg!(debug_assertions) {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}

mod resources {
    use bevy::prelude::*;

    #[derive(Debug, Resource, Default)]
    pub struct Counter(pub u32);
}

mod components {
    use bevy::prelude::*;

    #[derive(Debug, Component)]
    pub struct MyButton;

    #[derive(Debug, Component)]
    pub struct CounterText;

    #[derive(Debug, Component)]
    pub struct NormalDialog;

    #[derive(Debug, Component)]
    pub struct LargeDialog;
}
mod systems {
    use bevy::prelude::*;
    use oxidized_pixel_dungeon::user_interface::{
        components::Widget,
        widgets::{
            button1_widget, checkbox_widget, dialogbox1_widget, icon_widget,
            Button1WidgetPropsBuilder, CheckboxWidgetPropsBuilder, Dialogbox1WidgetPropsBuilder,
            Icon, IconWidgetPropsBuilder,
        },
        UserInterfaceAssets,
    };

    use crate::{
        components::{CounterText, LargeDialog, MyButton, NormalDialog},
        resources::Counter,
    };

    pub fn counter_system(
        mut counter: ResMut<Counter>,
        mut button_query: Query<&Interaction, (Changed<Interaction>, With<MyButton>)>,
        mut counter_text_query: Query<&mut Text, With<CounterText>>,
    ) {
        for interaction in button_query.iter_mut() {
            if interaction == &Interaction::Pressed {
                counter.0 += 1;
                let mut counter_text = counter_text_query.single_mut();
                *counter_text = Text::from_section(
                    format!("Counter: {}", counter.0),
                    TextStyle {
                        color: Color::WHITE,
                        ..default()
                    },
                );
            }
        }
    }

    pub fn layout_init_system(
        mut commands: Commands,
        user_interface_assets: Res<UserInterfaceAssets>,
        counter: Res<Counter>,
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
                parent.spawn((
                    TextBundle::from_section(
                        format!("Counter: {}", counter.0),
                        TextStyle {
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    CounterText,
                ));
                button1_widget(
                    parent,
                    &user_interface_assets,
                    MyButton,
                    Button1WidgetPropsBuilder::default()
                        .text("Button with Normal Text")
                        .build()
                        .unwrap(),
                );
                button1_widget(
                    parent,
                    &user_interface_assets,
                    MyButton,
                    Button1WidgetPropsBuilder::default()
                        .text("Button with Icon")
                        .icon_widget_props(Some(
                            IconWidgetPropsBuilder::default()
                                .icon(Icon::GoldCoins)
                                .build()
                                .unwrap(),
                        ))
                        .build()
                        .unwrap(),
                );
                button1_widget(
                    parent,
                    &user_interface_assets,
                    MyButton,
                    Button1WidgetPropsBuilder::default()
                        .text("Yellow text")
                        .font_color(Color::YELLOW)
                        .build()
                        .unwrap(),
                );
                button1_widget(
                    parent,
                    &user_interface_assets,
                    MyButton,
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
                    "Original Icon",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
                icon_widget(
                    parent,
                    &user_interface_assets,
                    IconWidgetPropsBuilder::default()
                        .icon(Icon::GoldCoins)
                        .size(32.0)
                        .build()
                        .unwrap(),
                );
                parent.spawn(TextBundle::from_section(
                    "Upscaled Icon",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
                checkbox_widget(
                    parent,
                    &user_interface_assets,
                    CheckboxWidgetPropsBuilder::default()
                        .text("Unchecked checkbox")
                        .build()
                        .unwrap(),
                );
                checkbox_widget(
                    parent,
                    &user_interface_assets,
                    CheckboxWidgetPropsBuilder::default()
                        .text("Checked checkbox")
                        .checked(true)
                        .build()
                        .unwrap(),
                );
                checkbox_widget(
                    parent,
                    &user_interface_assets,
                    CheckboxWidgetPropsBuilder::default()
                        .text("Bigger checkbox")
                        .checked(true)
                        .size(32.0)
                        .build()
                        .unwrap(),
                );
                dialogbox1_widget(
                    parent,
                    &user_interface_assets,
                    NormalDialog,
                    Dialogbox1WidgetPropsBuilder::default()
                        .visibility(Visibility::Inherited)
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle {
                                    text: Text::from_section(
                                        "This is a dialog box.",
                                        TextStyle {
                                            font: user_interface_assets
                                                .pixel_font_asset_handle
                                                .clone_weak(),
                                            font_size: 16.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                },
                                Widget,
                            ));
                        })
                        .build()
                        .unwrap(),
                );
                dialogbox1_widget(
                    parent,
                    &user_interface_assets,
                    LargeDialog,
                    Dialogbox1WidgetPropsBuilder::default()
                        .visibility(Visibility::Inherited)
                        .scale(4.0)
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle {
                                    text: Text::from_section(
                                        "This is a very big dialog box.",
                                        TextStyle {
                                            font: user_interface_assets
                                                .pixel_font_asset_handle
                                                .clone_weak(),
                                            font_size: 16.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                },
                                Widget,
                            ));
                        })
                        .build()
                        .unwrap(),
                );
            });
    }
}
