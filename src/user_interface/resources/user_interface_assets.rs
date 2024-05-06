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
use bevy_asset_loader::prelude::*;

#[derive(Debug, Resource, AssetCollection)]
pub struct UserInterfaceAssets {
    #[asset(path = "spd/fonts/pixel_font.ttf")]
    pub(crate) pixel_font_asset_handle: Handle<Font>,
    #[asset(path = "original/interfaces/button1.png")]
    pub(crate) button1_asset_handle: Handle<Image>,
    #[asset(path = "original/interfaces/icons/gold-coins.png")]
    pub(crate) gold_coin_icon_handle: Handle<Image>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum UserInterfaceAssetsLoadState {
    #[default]
    LoadingState,
    LoadedState,
}
