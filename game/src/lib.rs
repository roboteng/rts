use bevy::prelude::*;

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GameState>()
            .add_systems(Startup, draw_main_menu);
    }
}

fn draw_main_menu(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(TextBundle::from_section(
        "Hello, World!",
        TextStyle::default(),
    ));
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, States)]
enum GameState {
    #[default]
    MainMenu,
    Settings,
}
