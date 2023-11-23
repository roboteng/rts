use base::*;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use std::fmt::Display;

pub mod classes;
pub mod in_game;
pub mod main_menu;
pub mod settings_page;

#[derive(Default)]
pub struct BasePlugin(GameState, PlayType);
impl Plugin for BasePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GameState>()
            .add_systems(Startup, setup_2d)
            .insert_resource(MyUiScale::Medium)
            .add_state::<PlayType>();

        let mut k = app
            .world
            .get_resource_mut::<NextState<GameState>>()
            .unwrap();
        k.set(self.0);

        let mut l = app.world.get_resource_mut::<NextState<PlayType>>().unwrap();
        l.set(self.1);
    }
}

impl BasePlugin {
    pub fn new(init_state: GameState, play: PlayType) -> Self {
        Self(init_state, play)
    }
}

fn setup_2d(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 1,
            ..Default::default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
        },
        ..Default::default()
    });
}

fn button_interaction<T: Component>(
    query: Query<&Interaction, (Changed<Interaction>, With<T>)>,
    mut window: Query<&mut Window>,
) {
    for interaction in &query {
        for mut window in &mut window {
            window.cursor.icon = match interaction {
                Interaction::Pressed => CursorIcon::Default,
                Interaction::Hovered => CursorIcon::Hand,
                Interaction::None => CursorIcon::Default,
            }
        }
    }
}

pub fn teardown<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}

#[derive(Reflect, Resource, Clone, Copy, PartialEq)]
enum MyUiScale {
    Small,
    Medium,
    Large,
}

impl MyUiScale {
    pub fn items() -> Vec<Self> {
        vec![Self::Small, Self::Medium, Self::Large]
    }

    pub fn scale(&self) -> f32 {
        match self {
            Self::Small => 0.75,
            Self::Medium => 1.0,
            Self::Large => 1.5,
        }
    }
}

impl Display for MyUiScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Small => "Small",
            Self::Medium => "Medium",
            Self::Large => "Large",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test;
