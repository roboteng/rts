use bevy::prelude::*;

pub mod main_menu;

pub struct BasePlugin;
impl Plugin for BasePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GameState>().add_systems(Startup, setup_2d);
    }
}

fn setup_2d(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, States)]
enum GameState {
    #[default]
    MainMenu,
    Settings,
}
