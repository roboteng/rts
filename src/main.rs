use bevy::prelude::*;
use rts::CoreLogicPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CoreLogicPlugin))
        .run();
}
