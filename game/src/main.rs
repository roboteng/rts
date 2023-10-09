use bevy::{prelude::App, DefaultPlugins};
use game::BasePlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, BasePlugin)).run();
}
