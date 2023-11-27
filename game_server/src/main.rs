use base::*;
use bevy::prelude::*;
use game::{in_game::InGamePlugin, *};
use game_server::ServerPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BasePlugin::new(GameState::InGame, PlayType::Server),
            InGamePlugin,
            ServerPlugin
        ))
        .run();
}
