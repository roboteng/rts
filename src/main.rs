use bevy::prelude::*;
use rts::{core_logic::*, human_input::HumanInputPlugin, visuals::VisualsPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CoreLogicPlugin,
            VisualsPlugin,
            HumanInputPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut s: EventWriter<SpawnUnit>, mut commands: Commands) {
    let entity = commands.spawn_empty().id();
    s.send(SpawnUnit {
        target: entity,
        data: SpawnUnitData {
            pos: Vec2::new(0.0, 0.0),
        },
    });
}
