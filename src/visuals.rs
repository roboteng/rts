use crate::{core_logic::*, *};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub struct VisualsPlugin;
impl Plugin for VisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (create, show_selected));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        ..Default::default()
    });
}

fn create(
    mut events: EventReader<SpawnUnit>,
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
