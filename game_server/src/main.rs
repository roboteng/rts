use bevy::prelude::*;
use game::{in_game::InGamePlugin, *};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BasePlugin::new(GameState::MainMenu),
            InGamePlugin,
        ))
        .run();
}
