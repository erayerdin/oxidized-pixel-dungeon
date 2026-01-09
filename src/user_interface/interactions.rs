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

use super::{components::Interactable, UserInterfaceAssets};

pub trait OnPressed: Component + Sized {
    fn on_pressed(
        mut commands: Commands,
        user_interface_assets: Option<Res<UserInterfaceAssets>>,
        mut interaction_query: Query<
            (&Interaction, &mut Interactable),
            (Changed<Interaction>, With<Self>),
        >,
    ) {
        if let Some(ref user_interface_assets) = user_interface_assets {
            for (interaction, mut interactable) in interaction_query.iter_mut() {
                if *interaction == Interaction::Pressed && interactable.0 {
                    Self::invoke(&mut commands, user_interface_assets, &mut interactable);
                }
            }
        }
    }

    fn invoke(
        commands: &mut Commands,
        user_interface_assets: &Res<UserInterfaceAssets>,
        interactable: &mut Interactable,
    );
}

pub trait InteractionExt {
    fn register_on_pressed<T: OnPressed>(&mut self) -> &mut Self;
}

impl InteractionExt for App {
    fn register_on_pressed<T: OnPressed>(&mut self) -> &mut Self {
        self.add_systems(Update, T::on_pressed);
        self
    }
}
