use base::*;
use bevy::{prelude::App, DefaultPlugins};
use game::{
    in_game::{multiplayer::MultiplayerClientPlugin, InGamePlugin},
    main_menu::MainMenuPlugin,
    settings_page::SettingsPlugin,
    BasePlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BasePlugin::new(GameState::MainMenu, PlayType::None),
            MainMenuPlugin,
            SettingsPlugin,
            InGamePlugin,
            MultiplayerClientPlugin,
        ))
        .run();
}
