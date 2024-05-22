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
pub mod interactions;
pub mod resources;
pub mod systems;
pub mod widgets;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_parallax::ParallaxPlugin;

use crate::core::states::AppState;

pub use self::resources::{UserInterfaceAssets, UserInterfaceAssetsLoadState};
use self::{
    interactions::InteractionExt,
    systems::{checkbox_check_system, main_menu},
};

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ParallaxPlugin)
            .init_state::<UserInterfaceAssetsLoadState>()
            .add_loading_state(
                LoadingState::new(UserInterfaceAssetsLoadState::default())
                    .continue_to_state(UserInterfaceAssetsLoadState::LoadedState)
                    .load_collection::<UserInterfaceAssets>(),
            )
            .add_systems(
                OnEnter(UserInterfaceAssetsLoadState::LoadedState),
                (
                    main_menu::ui_init_system.run_if(in_state(AppState::MainMenu)),
                    main_menu::parallax_init_system.run_if(in_state(AppState::MainMenu)),
                ),
            )
            .add_systems(
                Update,
                (main_menu::parallax_play_system, checkbox_check_system),
            )
            .register_on_pressed::<main_menu::StartTheGameButton>();
    }
}
