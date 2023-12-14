use bevy::prelude::*;

pub struct CoreLogicPlugin;
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
#[derive(Bundle)]
struct MyBundle {
    transform: Transform,
    commands: UserCommandComponent,
}

#[derive(Component)]
enum UnitComponents {
    #[allow(unused)]
    Villager,
}

#[derive(Component, Debug, Default, PartialEq, Eq)]
struct UserCommandComponent {
    command: Option<UserCommand>,
}

#[derive(Debug, PartialEq, Eq)]
enum UserCommand {}

fn create_unit_bundles(spawn: &SpawnUnitData) -> MyBundle {
    let transform = Transform {
        translation: Vec3::new(spawn.pos.x, spawn.pos.y, 0.0),
        ..Default::default()
    };

    MyBundle {
        transform,
        commands: UserCommandComponent::default(),
    }
}

#[derive(Debug, Event)]
struct SpawnUnit {
    target: Entity,
    data: SpawnUnitData,
}

#[derive(Debug)]
struct SpawnUnitData {
    pos: Vec2,
    #[allow(unused)]
    unit: Unit,
}

#[derive(Debug)]
enum Unit {
    #[allow(unused)]
    Villager,
}

impl SpawnUnit {
    #[allow(unused)]
    pub fn new(target: Entity, data: SpawnUnitData) -> Self {
        Self { target, data }
    }
}

#[derive(Debug, Event)]
enum PlayerCommand {
    #[allow(unused)]
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

            assert_eq!(Vec3::new(3.0, 4.0, 0.0), actual.transform.translation);
        }

        #[test]
        fn user_commands_get_created() {
            let actual = create_unit_bundles(&SpawnUnitData {
                pos: Vec2::default(),
                unit: Unit::Villager,
            });

            assert_eq!(UserCommandComponent::default(), actual.commands);
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
