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
    // Fonts
    #[asset(path = "spd/fonts/pixel_font.ttf")]
    pub(crate) pixel_font_asset_handle: Handle<Font>,
    // UI Elements
    #[asset(path = "original/interfaces/button1.png")]
    pub(crate) button1_asset_handle: Handle<Image>,
    #[asset(path = "original/interfaces/checkbox-unchecked.png")]
    pub(crate) checkbox_unchecked_asset_handle: Handle<Image>,
    #[asset(path = "original/interfaces/checkbox-checked.png")]
    pub(crate) checkbox_checked_asset_handle: Handle<Image>,
    // Icons
    #[asset(path = "original/interfaces/icons/gold-coins.png")]
    pub(crate) gold_coin_icon_handle: Handle<Image>,
    #[asset(path = "original/interfaces/icons/cogs.png")]
    pub(crate) cogs_icon_handle: Handle<Image>,
    #[asset(path = "original/interfaces/icons/crest-of-blade.png")]
    pub(crate) crest_of_blade_icon_handle: Handle<Image>,
    #[asset(path = "original/interfaces/icons/downstairs.png")]
    pub(crate) downstairs_icon_handle: Handle<Image>,
    #[asset(path = "original/interfaces/icons/front-facing-scroll.png")]
    pub(crate) front_facing_scroll_icon_handle: Handle<Image>,
    #[asset(path = "original/interfaces/icons/ouroboros-arrows.png")]
    pub(crate) ouroboros_arrows_icon_handle: Handle<Image>,
    #[asset(path = "original/interfaces/icons/ranking-ladders.png")]
    pub(crate) ranking_ladders_icon_handle: Handle<Image>,
    #[asset(path = "original/interfaces/icons/shattered-block.png")]
    pub(crate) shattered_block_icon_handle: Handle<Image>,
    // Images
    #[asset(path = "original/interfaces/banner1.png")]
    pub(crate) banner1_handle: Handle<Image>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum UserInterfaceAssetsLoadState {
    #[default]
    LoadingState,
    LoadedState,
}
