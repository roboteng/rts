use bevy::prelude::*;
use bevy_ui_dsl::*;

use crate::{classes::*, *};

pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Settings), draw_settings)
            .add_systems(
                Update,
                (button_actions, button_interaction::<SettingsButton>)
                    .run_if(in_state(GameState::Settings)),
            )
            .add_systems(OnExit(GameState::Settings), teardown::<OnSettings>);
    }
}

fn draw_settings(mut commands: Commands, assets: Res<AssetServer>) {
    rooti(
        (c_background, c_centers),
        &assets,
        &mut commands,
        OnSettings,
        |p| {
            node((), p, |p| {
                buttoni(c_button, SettingsButton::Back, p, |p| {
                    text("Back", (), c_button_text, p);
                });
            });
        },
    );
}

fn button_actions(
    query: Query<(&Interaction, &SettingsButton), Changed<Interaction>>,
    mut state: ResMut<NextState<GameState>>,
) {
    for (interaction, button) in &query {
        if interaction == &Interaction::Pressed {
            match button {
                SettingsButton::Back => state.set(GameState::MainMenu),
            }
        }
    }
}

#[derive(Component)]
enum SettingsButton {
    Back,
}

#[derive(Component)]
struct OnSettings;
