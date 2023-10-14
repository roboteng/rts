use bevy::prelude::*;

pub mod classes;
pub mod main_menu;
pub mod settings_page;

pub struct BasePlugin;
impl Plugin for BasePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GameState>().add_systems(Startup, setup_2d);
    }
}

fn setup_2d(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, States)]
pub(crate) enum GameState {
    #[default]
    MainMenu,
    Settings,
}

#[cfg(test)]
mod test;
