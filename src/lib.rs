use bevy::prelude::*;

struct CoreLogicPlugin;
impl Plugin for CoreLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnUnit>();

        app.add_systems(Update, foo);
    }
}

fn foo(mut creations: EventReader<SpawnUnit>, mut commands: Commands) {
    for spawn in creations.read() {
        match spawn {
            SpawnUnit::Villager(entity, pos) => commands.entity(*entity).insert(Transform {
                translation: Vec3::new(pos.x, pos.y, 0.0),
                ..Default::default()
            }),
        };
    }
}

#[derive(Debug, Event)]
enum SpawnUnit {
    Villager(Entity, Vec2),
}

#[derive(Debug, Event)]
enum PlayerCommand {
    Move(Entity, Vec2),
}

#[cfg(test)]
mod test {
    use super::*;
    mod acceptance {
        use std::time::Duration;

        use super::*;

        #[test]
        fn move_a_villager() {
            // arrange
            let mut app = App::new();

            app.add_plugins((MinimalPlugins, CoreLogicPlugin));

            let ent = app.world.spawn_empty().id();

            let init_pos = Vec2::new(1.0, 1.0);
            app.world.send_event(SpawnUnit::Villager(ent, init_pos));

            // act
            let goal_pos = Vec2::new(0.0, 0.0);
            app.world.send_event(PlayerCommand::Move(ent, goal_pos));
            app.update();

            // assert
            loop {
                {
                    let actual = app.world.get::<Transform>(ent).unwrap();
                    if actual.translation.length() < init_pos.length() {
                        break;
                    }
                }

                let mut time = app.world.get_resource_mut::<Time<Virtual>>();
                let time = time.as_mut().unwrap();
                time.advance_by(Duration::from_millis(16));
                if time.elapsed_seconds() > 5.0 {
                    panic!("Villager didn't move closer to the goal")
                }
                app.update();
            }
        }
    }
}
