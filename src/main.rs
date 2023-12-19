use bevy::prelude::*;
use rts::{CoreLogicPlugin, MoveToCommand, SpawnVillager, SpawnVillagerData, Speed};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CoreLogicPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (create, handle_input))
        .run();
}

fn setup(mut s: EventWriter<SpawnVillager>, mut commands: Commands) {
    let entity = commands.spawn_empty().id();
    s.send(SpawnVillager {
        target: entity,
        data: SpawnVillagerData {
            pos: Vec2::new(0.0, 0.0),
        },
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..Default::default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            color: Color::WHITE,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(2.0, 3.0, 10.0)),
        ..Default::default()
    });
}

fn create(
    mut events: EventReader<SpawnVillager>,
    transforms: Query<&Transform>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for event in events.read() {
        if let Some(mut ent) = commands.get_entity(event.target) {
            ent.insert(PbrBundle {
                material: materials.add(Color::WHITE.into()),
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                transform: *transforms
                    .get(event.target)
                    .unwrap_or(&Transform::default()),
                ..Default::default()
            });
        }
    }
}

fn handle_input(
    key: Res<Input<KeyCode>>,
    mut events: EventWriter<MoveToCommand>,
    entities: Query<Entity, With<Speed>>,
) {
    if key.pressed(KeyCode::Space) {
        if let Some(entity) = entities.iter().next() {
            events.send(MoveToCommand {
                target: entity,
                destination: Vec2::new(5.0, 5.0),
            })
        }
    }
}
