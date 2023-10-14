use bevy::{prelude::App, DefaultPlugins};
use game::{main_menu::MainMenuPlugin, settings_page::SettingsPlugin, BasePlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BasePlugin, MainMenuPlugin, SettingsPlugin))
        .run();
}
