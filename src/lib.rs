use bevy::prelude::*;

pub struct CoreLogicPlugin;
impl Plugin for CoreLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnVillager>();
        app.add_event::<MoveToCommand>();

        app.add_systems(PreUpdate, spawn_villagers);
        app.add_systems(Update, give_commands);
    }
}

fn spawn_villagers(mut creations: EventReader<SpawnVillager>, mut commands: Commands) {
    for spawn in creations.read() {
        let bundle = spawn.data.to_bundle();
        commands.entity(spawn.target).insert(bundle);
    }
}

fn give_commands(
    mut incoming_commands: EventReader<MoveToCommand>,
    mut query: Query<(&mut UnitCommandsComponent, &mut Transform)>,
) {
    for comm in incoming_commands.read() {
        let MoveToCommand {
            target,
            destination,
        } = comm;
        if let Ok((_, mut transform)) = query.get_mut(*target) {
            let transform = transform.as_mut();

            transform.translation.x = destination.x;
            transform.translation.y = destination.y;
        }
    }
}

#[derive(Bundle)]
struct VillagerBundle {
    transform: Transform,
    commands: UnitCommandsComponent,
}

#[derive(Component, Debug, Default, PartialEq, Eq)]
struct UnitCommandsComponent {
    command: Option<UserCommand>,
}

#[derive(Debug, PartialEq, Eq)]
enum UserCommand {}

#[derive(Debug, Event)]
struct SpawnVillager {
    target: Entity,
    data: SpawnVillagerData,
}

#[derive(Debug)]
struct SpawnVillagerData {
    pos: Vec2,
}

impl SpawnVillagerData {
    fn to_bundle(&self) -> VillagerBundle {
        let transform = Transform {
            translation: Vec3::new(self.pos.x, self.pos.y, 0.0),
            ..Default::default()
        };

        VillagerBundle {
            transform,
            commands: UnitCommandsComponent::default(),
        }
    }
}

#[derive(Debug, Event)]
struct MoveToCommand {
    target: Entity,
    destination: Vec2,
}

impl MoveToCommand {
    pub fn new(target: Entity, destination: Vec2) -> Self {
        Self {
            target,
            destination,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    mod unit_test {
        use super::*;

        #[test]
        fn transfor_gets_created() {
            let actual = &SpawnVillagerData {
                pos: Vec2 { x: 3.0, y: 4.0 },
            }
            .to_bundle();

            assert_eq!(Vec3::new(3.0, 4.0, 0.0), actual.transform.translation);
        }

        #[test]
        fn user_commands_get_created() {
            let actual = &SpawnVillagerData {
                pos: Vec2::default(),
            }
            .to_bundle();

            assert_eq!(UnitCommandsComponent::default(), actual.commands);
        }

        #[test]
        fn assign_commands() {}
    }

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
            app.world.send_event(SpawnVillager {
                target: ent,
                data: SpawnVillagerData { pos: init_pos },
            });

            // act
            let goal_pos = Vec2::new(0.0, 0.0);
            app.world.send_event(MoveToCommand::new(ent, goal_pos));

            // assert
            assert_timeout(
                &mut app,
                |app| {
                    if let Some(actual) = app.world.get::<Transform>(ent) {
                        actual.translation.length() < init_pos.length()
                    } else {
                        false
                    }
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
