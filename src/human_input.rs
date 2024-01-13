use crate::{core_logic::*, *};

pub struct HumanInputPlugin;
impl Plugin for HumanInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_unit, select_unit));
    }
}

fn find_pointer_position(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) -> Option<Vec2> {
    let (camera, camera_transform) = camera_query.iter().next()?;

    let cursor_position = windows.single().cursor_position()?;

    camera.viewport_to_world_2d(camera_transform, cursor_position)
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
