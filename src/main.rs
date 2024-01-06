use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rts::{CoreLogicPlugin, MoveToCommand, SpawnVillager, SpawnVillagerData, Speed, Vec3Extension};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CoreLogicPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (create, move_unit, select_unit, show_selected))
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

    commands.spawn(Camera2dBundle {
        ..Default::default()
    });
}

fn create(
    mut events: EventReader<SpawnVillager>,
    transforms: Query<&Transform>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Build a default quad mesh
    let mut mesh = Mesh::from(shape::Quad::default());
    // Build vertex colors for the quad. One entry per vertex (the corners of the quad)
    let vertex_colors: Vec<[f32; 4]> = vec![
        Color::RED.as_rgba_f32(),
        Color::GREEN.as_rgba_f32(),
        Color::BLUE.as_rgba_f32(),
        Color::WHITE.as_rgba_f32(),
    ];
    // Insert the vertex colors as an attribute
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);

    let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();

    for event in events.read() {
        if let Some(mut ent) = commands.get_entity(event.target) {
            ent.insert(MaterialMesh2dBundle {
                mesh: mesh_handle.clone(),
                transform: transforms
                    .get(event.target)
                    .unwrap_or(&Transform::default())
                    .with_scale(Vec3::splat(128.)),
                material: materials.add(ColorMaterial::default()),
                ..default()
            });
        }
    }
}

fn move_unit(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    clicks: Res<Input<MouseButton>>,
    mut events: EventWriter<MoveToCommand>,
    entities: Query<Entity, With<Selected>>,
) {
    if clicks.just_pressed(MouseButton::Right) {
        let Some(point) = find_pointer_position(camera_query, windows) else {
            return;
        };

        if let Some(entity) = entities.iter().next() {
            events.send(MoveToCommand {
                target: entity,
                destination: point,
            })
        }
    }
}

#[derive(Component)]
struct Selected;

fn find_pointer_position(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) -> Option<Vec2> {
    let (camera, camera_transform) = camera_query.iter().next()?;

    let cursor_position = windows.single().cursor_position()?;

    camera.viewport_to_world_2d(camera_transform, cursor_position)
}

fn select_unit(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    clicks: Res<Input<MouseButton>>,
    entities: Query<(&Transform, Entity), With<Speed>>,
) {
    let Some(point) = find_pointer_position(camera_query, windows) else {
        return;
    };

    if clicks.pressed(MouseButton::Left) {
        let mut any_clicked = false;
        for (transform, entity) in entities.iter() {
            let bl = transform.translation.to_vec2() - transform.scale.to_vec2() / 2.0;
            let tr = transform.translation.to_vec2() + transform.scale.to_vec2() / 2.0;
            if bl.x < point.x && point.x < tr.x && bl.y < point.y && point.y < tr.y {
                commands.entity(entity).insert(Selected);
                any_clicked = true;
            }
        }
        if !any_clicked {
            for (_, entity) in entities.iter() {
                commands.entity(entity).remove::<Selected>();
            }
        }
    }
}

fn show_selected(selctions: Query<&Transform, With<Selected>>, mut gizmos: Gizmos) {
    for selection in selctions.iter() {
        gizmos.rect_2d(
            selection.translation.to_vec2(),
            0.0,
            selection.scale.to_vec2(),
            Color::WHITE,
        );
    }
}
