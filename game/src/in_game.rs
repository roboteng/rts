use bevy::prelude::*;
use bevy_ui_dsl::*;

use crate::GameState;

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (setup, draw_hud));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        camera: Camera {
            order: 1,
            ..Default::default()
        },
        transform: Transform::from_xyz(8.0, 8.0, 7.0).looking_at(Vec3::splat(0.0), Vec3::Y),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::WHITE,
            intensity: 1000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, 0.0),
        ..Default::default()
    });

    let mesh = meshes.add(
        shape::Plane {
            size: 20.0,
            subdivisions: 0,
        }
        .into(),
    );
    let material = materials.add(Color::ALICE_BLUE.into());

    commands.spawn(PbrBundle {
        mesh,
        material,
        ..Default::default()
    });

    let box_mesh = meshes.add(shape::Box::new(0.8, 1.0, 0.8).into());
    let box_material = materials.add(Color::SEA_GREEN.into());

    for i in 0..2 {
        commands.spawn(PbrBundle {
            mesh: box_mesh.clone(),
            material: box_material.clone(),
            transform: Transform::from_xyz(i as f32, 0.0, i as f32),
            ..Default::default()
        });
    }
}

fn c_root(n: &mut NodeBundle) {
    n.style.flex_direction = FlexDirection::ColumnReverse;
}

fn c_hud(n: &mut NodeBundle) {
    n.style.width = Val::Vw(100.0);
    n.style.height = Val::Vh(20.0);
    n.background_color = Color::MAROON.into();
}

fn draw_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    root(c_root, &asset_server, &mut commands, |p| {
        node(c_hud, p, |_| {});
    });
}
