use bevy::{prelude::App, DefaultPlugins};
use game::{main_menu::MainMenuPlugin, BasePlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BasePlugin, MainMenuPlugin))
        .run();
}
