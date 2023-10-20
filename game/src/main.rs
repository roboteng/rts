use bevy::{prelude::App, DefaultPlugins};
use game::{main_menu::MainMenuPlugin, settings_page::SettingsPlugin, BasePlugin, GameState};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BasePlugin::new(GameState::InGame),
            MainMenuPlugin,
            SettingsPlugin,
        ))
        .run();
}
