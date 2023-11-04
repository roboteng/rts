use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_ui_dsl::*;

use crate::GameState;

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins)
            .add_systems(OnEnter(GameState::InGame), (setup, draw_hud))
            .add_systems(
                Update,
                (apply_seelected, click_on_ground, process_user_commands),
            );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(({
        let mut camera = Camera3dBundle::default();
        camera.camera.order = 0;
        camera.transform = Transform::from_xyz(8.0, 8.0, 7.0).looking_at(Vec3::ZERO, Vec3::Y);
        camera
    },));

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

    commands.spawn((
        PbrBundle {
            mesh,
            material,
            ..Default::default()
        },
        Ground,
    ));

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
            Health {
                max: 5.0,
                current: (i + 2) as f32,
            },
            UserCommands(Vec::new()),
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
struct Ground;

#[derive(Component)]
struct HealthVis;

#[derive(Component)]
struct UserCommands(Vec<Vec3>);

#[derive(Component)]
struct Health {
    max: f32,
    current: f32,
}

fn apply_seelected(
    q: Query<(&Health, &PickSelection)>,
    mut texts: Query<&mut Text, With<HealthVis>>,
) {
    let selections = q
        .iter()
        .filter(|(_, s)| s.is_selected)
        .map(|(h, _)| h)
        .collect::<Vec<_>>();
    for mut text in &mut texts {
        if selections.len() == 1 {
            let health = selections[0];
            text.sections = vec![TextSection {
                value: format!("{} of {}", health.current, health.max),
                style: TextStyle::default(),
            }];
        } else {
            text.sections = vec![];
        }
    }
}

fn click_on_ground(
    grounds: Query<Entity, With<Ground>>,
    mut ev: EventReader<Pointer<Click>>,
    mut selecteds: Query<(&mut UserCommands, &PickSelection)>,
) {
    if !ev.is_empty() {
        if let Some(ground) = grounds.iter().next() {
            for e in &mut ev {
                if e.target == ground {
                    match e.button {
                        PointerButton::Primary => {}
                        PointerButton::Secondary => {
                            for (mut selected, selection) in &mut selecteds {
                                if !selection.is_selected {
                                    continue;
                                }
                                selected.0.push(e.hit.position.unwrap());
                            }
                        }
                        PointerButton::Middle => {}
                    }
                }
            }
        }
    }
}

fn process_user_commands(mut actions: Query<(&mut Transform, &mut UserCommands)>) {
    for (mut trans, mut comms) in &mut actions {
        if comms.0.is_empty() {
            continue;
        }
        trans.translation = comms.0[0];
        comms.0.clear();
    }
}
