use bevy::prelude::*;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    Settings,
}

#[derive(Debug, Default, States, Hash, PartialEq, Eq, Clone, Copy)]
pub enum PlayType {
    #[default]
    None,
    Single,
    Multi,
    Server,
}
