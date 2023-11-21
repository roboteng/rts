use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub mod logic;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    Settings,
}

#[derive(Debug, Default, States, Hash, PartialEq, Eq, Clone, Copy)]
pub enum PlayType {
    #[default]
    None,
    Single,
    Multi,
    Server,
}

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct HealthVis;

#[derive(Component)]
pub struct UserCommands(Vec<Vec3>);

#[derive(Component)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

#[derive(Component, PartialEq, Eq, Clone, Copy, Debug)]
pub struct NetId(pub u64);

#[derive(Event)]
pub struct SelectEvent {
    pub pos: Vec3,
    pub entity: NetId,
}

pub fn setup(
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
        NetId(0),
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
            PickableBundle {
                pickable: Pickable {
                    should_block_lower: false,
                    should_emit_events: true,
                },
                ..Default::default()
            },
            Health {
                max: 5.0,
                current: (i + 2) as f32,
            },
            UserCommands(Vec::new()),
            NetId(i + 1),
        ));
    }
}
