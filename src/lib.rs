use bevy::prelude::*;

struct CoreLogicPlugin;
impl Plugin for CoreLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnUnit>();

        app.add_systems(Update, spawn_units);
    }
}

fn spawn_units(mut creations: EventReader<SpawnUnit>, mut commands: Commands) {
    for spawn in creations.read() {
        let bundle = create_unit_bundles(&spawn.data);
        commands.entity(spawn.target).insert(bundle);
    }
}

#[derive(Component)]
enum UnitComponents {
    Villager,
}

fn create_unit_bundles(spawn: &SpawnUnitData) -> Transform {
    let transform = Transform {
        translation: Vec3::new(spawn.pos.x, spawn.pos.y, 0.0),
        ..Default::default()
    };

    transform
}

#[derive(Debug, Event)]
struct SpawnUnit {
    target: Entity,
    data: SpawnUnitData,
}

#[derive(Debug)]
struct SpawnUnitData {
    pos: Vec2,
    unit: Unit,
}

#[derive(Debug)]
enum Unit {
    Villager,
}

impl SpawnUnit {
    pub fn new(target: Entity, data: SpawnUnitData) -> Self {
        Self { target, data }
    }
}

#[derive(Debug, Event)]
enum PlayerCommand {
    Move(Entity, Vec2),
}

#[cfg(test)]
mod test {
    use super::*;
    mod unit_test {
        use super::*;

        #[test]
        fn transfor_gets_created() {
            let actual = create_unit_bundles(&SpawnUnitData {
                pos: Vec2 { x: 3.0, y: 4.0 },
                unit: Unit::Villager,
            });

            assert_eq!(Vec3::new(3.0, 4.0, 0.0), actual.translation);
        }
    }

    mod acceptance {
        use std::time::Duration;

        use super::*;

        #[ignore = "In dev"]
        #[test]
        fn move_a_villager() {
            // arrange
            let mut app = App::new();

            app.add_plugins((MinimalPlugins, CoreLogicPlugin));

            let ent = app.world.spawn_empty().id();

            let init_pos = Vec2::new(1.0, 1.0);
            app.world.send_event(SpawnUnit::new(
                ent,
                SpawnUnitData {
                    pos: init_pos,
                    unit: Unit::Villager,
                },
            ));

            // act
            let goal_pos = Vec2::new(0.0, 0.0);
            app.world.send_event(PlayerCommand::Move(ent, goal_pos));
            app.update();

            // assert
            assert_timeout(
                &mut app,
                |app| {
                    let actual = app.world.get::<Transform>(ent).unwrap();
                    actual.translation.length() < init_pos.length()
                },
                "Villager didn't move closer to the goal",
            );
        }

        fn assert_timeout(
            app: &mut App,
            success_condition: impl Fn(&App) -> bool,
            failure_message: &'static str,
        ) {
            loop {
                {
                    if success_condition(app) {
                        break;
                    }
                }
                timeout(app, failure_message);
            }
        }

        fn timeout(app: &mut App, message: &'static str) {
            let mut time = app.world.get_resource_mut::<Time<Virtual>>();
            let time = time.as_mut().unwrap();
            time.advance_by(Duration::from_millis(16));
            if time.elapsed() > Duration::from_secs(5) {
                panic!("{}", message);
            }
            app.update();
        }
    }
}
