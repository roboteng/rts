use bevy::{prelude::App, DefaultPlugins};
use game::{
    in_game::InGamePlugin, main_menu::MainMenuPlugin, settings_page::SettingsPlugin, BasePlugin,
    GameState,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BasePlugin::new(GameState::MainMenu),
            MainMenuPlugin,
            SettingsPlugin,
            InGamePlugin,
        ))
        .run();
}
