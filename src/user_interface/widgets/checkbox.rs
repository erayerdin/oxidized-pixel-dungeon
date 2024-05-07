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

use crate::user_interface::UserInterfaceAssets;

#[derive(Debug, Builder)]
pub struct CheckboxWidgetProps {
    #[builder(setter(custom))]
    text: String,
    #[builder(default = "false")]
    checked: bool,
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
    let CheckboxWidgetProps { text, checked } = props;
}
