use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_ui_dsl::*;

use crate::GameState;

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins)
            .add_systems(OnEnter(GameState::InGame), (setup, draw_hud))
            .add_systems(Update, apply_seelected);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        {
            let mut camera = Camera3dBundle::default();
            camera.camera.order = 0;
            camera.transform = Transform::from_xyz(8.0, 8.0, 7.0).looking_at(Vec3::ZERO, Vec3::Y);
            camera
        },
        RaycastPickCamera::default(),
    ));

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
            size: 10.0,
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
        commands.spawn((
            PbrBundle {
                mesh: box_mesh.clone(),
                material: box_material.clone(),
                transform: Transform::from_xyz(i as f32, 0.0, i as f32),
                ..Default::default()
            },
            PickableBundle::default(),
            RaycastPickTarget::default(),
            Health {
                max: 5.0,
                current: (i + 2) as f32,
            },
        ));
    }
}

fn c_hud(n: &mut NodeBundle) {
    n.style.width = Val::Vw(100.0);
    n.style.height = Val::Vh(20.0);
    n.style.top = Val::Vh(80.0);
    n.style.position_type = PositionType::Absolute;
    n.background_color = Color::MAROON.into();
}

fn draw_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    rooti(
        c_hud,
        &asset_server,
        &mut commands,
        (
            Hud,
            Pickable {
                should_block_lower: true,
                should_emit_events: false,
            },
        ),
        |p| {
            texti("", (), (), HealthVis, p);
        },
    );
}

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HealthVis;

#[derive(Component)]
struct Health {
    max: f32,
    current: f32,
}

fn apply_seelected(
    q: Query<(&Health, &PickingInteraction), Changed<PickingInteraction>>,
    mut texts: Query<&mut Text, With<HealthVis>>,
) {
    for (health, interaction) in &q {
        for mut text in &mut texts {
            match interaction {
                PickingInteraction::Pressed => {
                    text.sections = vec![TextSection {
                        value: format!("{} of {}", health.current, health.max),
                        style: TextStyle::default(),
                    }]
                }
                PickingInteraction::Hovered => {}
                PickingInteraction::None => text.sections = vec![],
            }
        }
    }
}
