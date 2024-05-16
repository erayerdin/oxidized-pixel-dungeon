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

pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::mob::{
    components::{HeroClass, HeroTier},
    resources::{HeroAssets, HeroAssetsLoadState},
    systems::{hero_init_system, hero_texture_change_system, hero_tier_change_system},
};

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        debug!("Initializing MobPlugin...");
        app.register_type::<HeroClass>()
            .register_type::<HeroTier>()
            .init_state::<HeroAssetsLoadState>()
            .add_loading_state(
                LoadingState::new(HeroAssetsLoadState::Loading)
                    .continue_to_state(HeroAssetsLoadState::Loaded)
                    .load_collection::<HeroAssets>(),
            )
            .add_systems(OnEnter(HeroAssetsLoadState::Loaded), hero_init_system)
            .add_systems(Update, hero_tier_change_system);

        if cfg!(debug_assertions) {
            app.add_systems(Update, hero_texture_change_system);
        }
    }
}
