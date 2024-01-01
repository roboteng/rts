use bevy::prelude::*;

pub struct CoreLogicPlugin;
impl Plugin for CoreLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnVillager>();
        app.add_event::<MoveToCommand>();

        app.add_systems(PreUpdate, spawn_villagers);
        app.add_systems(Update, (give_commands, move_units));
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
    mut query: Query<&mut UnitCommandsComponent>,
) {
    for comm in incoming_commands.read() {
        let MoveToCommand {
            target,
            destination,
        } = comm;
        if let Ok(mut unit_commands) = query.get_mut(*target) {
            let k = unit_commands.as_mut();
            *k = UnitCommandsComponent {
                command: Some(UnitCommand::MoveTo(*destination)),
            };
        }
    }
}

trait Vec3Extension {
    fn to_vec2(&self) -> Vec2;
}

trait Vec2Extension {
    fn to_vec3(&self) -> Vec3;
}

impl Vec3Extension for Vec3 {
    fn to_vec2(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Vec2Extension for Vec2 {
    fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: 0.0,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Step {
    Continue(Vec3),
    Stop(Vec3),
}

fn next_step(current: Vec3, goal: Vec3, max_step: f32) -> Step {
    let delta = goal - current;
    if delta.length() > max_step {
        Step::Continue(delta.normalize() * max_step + current)
    } else {
        Step::Stop(goal)
    }
}

fn move_units(mut q: Query<(&mut UnitCommandsComponent, &mut Transform, &Speed)>, time: Res<Time>) {
    for (mut cmds, mut transform, speed) in q.iter_mut() {
        match cmds.as_mut().command {
            Some(UnitCommand::MoveTo(pos)) => {
                match next_step(
                    transform.translation,
                    pos.to_vec3(),
                    time.delta_seconds() * speed.0,
                ) {
                    Step::Continue(step) => {
                        transform.translation = step;
                    }
                    Step::Stop(step) => {
                        cmds.command = None;
                        transform.translation = step;
                    }
                }
            }
            None => {}
        };
    }
}

#[derive(Bundle)]
struct VillagerBundle {
    transform: Transform,
    commands: UnitCommandsComponent,
    speed: Speed,
}

#[derive(Component, Debug, Default, PartialEq)]
struct UnitCommandsComponent {
    command: Option<UnitCommand>,
}

#[derive(Debug, PartialEq)]
enum UnitCommand {
    MoveTo(Vec2),
}

#[derive(Component)]
pub struct Speed(f32);

#[derive(Debug, Event)]
pub struct SpawnVillager {
    pub target: Entity,
    pub data: SpawnVillagerData,
}

#[derive(Debug)]
pub struct SpawnVillagerData {
    pub pos: Vec2,
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
            speed: Speed(5.0),
        }
    }
}

#[derive(Debug, Event)]
pub struct MoveToCommand {
    pub target: Entity,
    pub destination: Vec2,
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
        fn transform_gets_created() {
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
        fn test_next_step_from_zero() {
            let cases = &[
                (Vec3::X * 10.0, 1.0, Step::Continue(Vec3::X)),
                (Vec3::X, 1.0, Step::Stop(Vec3::X)),
                (Vec3::X, 2.0, Step::Stop(Vec3::X)),
            ];
            cases.iter().for_each(|(goal, step, expected)| {
                let actual = next_step(Vec3::ZERO, *goal, *step);

                assert_eq!(actual, *expected);
            });
        }

        #[test]
        fn test_next_step_from_x() {
            let goal = Vec3::X + Vec3::Y;
            let current = Vec3::X;
            let actual = next_step(current, goal, 1.0);
            let expected = Step::Stop(goal);

            assert_eq!(actual, expected);
        }
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
