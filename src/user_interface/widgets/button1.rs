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

use super::{icon::IconWidgetProps, icon_widget};

const BUTTON_PADDING_PX_VAL: f32 = 8.0;
const BUTTON_PADDING_UI_RECT: UiRect = UiRect::all(Val::Px(BUTTON_PADDING_PX_VAL));
const BUTTON_CORNER_SCALE: f32 = BUTTON_PADDING_PX_VAL / 8.0;

#[derive(Debug, Builder)]
pub struct Button1WidgetProps {
    #[builder(setter(custom))]
    text: String,
    #[builder(default = "None")]
    icon_widget_props: Option<IconWidgetProps>,
    #[builder(default = "Color::WHITE")]
    font_color: Color,
    #[builder(default = "16.0")]
    font_size: f32,
}

impl Button1WidgetPropsBuilder {
    pub fn text(&mut self, text: impl Into<String>) -> &mut Self {
        self.text = Some(text.into());
        self
    }
}

pub fn button1_widget(
    parent: &mut ChildBuilder,
    user_interface_assets: &Res<UserInterfaceAssets>,
    marker_component: impl Component,
    props: Button1WidgetProps,
) {
    let Button1WidgetProps {
        text,
        icon_widget_props,
        font_color,
        font_size,
    } = props;

    parent
        .spawn((
            NodeBundle {
                background_color: Color::BLACK.with_a(0.5).into(),
                style: Style {
                    width: Val::Auto,
                    height: Val::Auto,
                    ..default()
                },
                ..default()
            },
            Name::new("Button1"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Auto,
                            height: Val::Auto,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            padding: BUTTON_PADDING_UI_RECT,
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
                        max_corner_scale: BUTTON_CORNER_SCALE,
                    }),
                    marker_component,
                    Widget,
                ))
                .with_children(|parent| {
                    let mut text_margin = UiRect::ZERO;

                    if let Some(icon_widget_props) = icon_widget_props {
                        icon_widget(parent, user_interface_assets, icon_widget_props);
                        text_margin = UiRect::left(Val::Px(8.0));
                    }

                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    text,
                                    TextStyle {
                                        font: user_interface_assets
                                            .pixel_font_asset_handle
                                            .clone_weak(),
                                        font_size,
                                        color: font_color,
                                    },
                                )],
                                justify: JustifyText::Center,
                                linebreak_behavior: bevy::text::BreakLineOn::WordBoundary,
                            },
                            style: Style {
                                margin: text_margin,
                                ..default()
                            },
                            ..default()
                        },
                        Widget,
                    ));
                });
        });
}
