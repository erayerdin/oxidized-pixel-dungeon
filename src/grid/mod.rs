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

use bevy::{
    app::{Plugin, Startup},
    asset::Assets,
    ecs::system::{Commands, ResMut},
    log::debug,
    math::primitives::Rectangle,
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
    transform::components::Transform,
};

pub(super) struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        debug!("Initializing GridPlugin...");
        app.add_systems(Startup, init_system);
    }
}

fn init_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    debug!("Drawing square...");

    // REF: https://bevyengine.org/examples/2D%20Rendering/2d-shapes/
    const X_EXTENT: f32 = 600.;

    let mesh_handler = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0)));
    let color = Color::hsl(360. * 0. / 1., 0.95, 0.7);

    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_handler,
        material: materials.add(color),
        transform: Transform::from_xyz(
            // Distribute shapes from -X_EXTENT to +X_EXTENT.
            -X_EXTENT / 2. + 0. * X_EXTENT,
            0.0,
            0.0,
        ),
        ..Default::default()
    });
}
