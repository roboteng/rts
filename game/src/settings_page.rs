use bevy::prelude::*;
use bevy_ui_dsl::*;

use crate::{classes::*, *};

pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Settings), draw_settings)
            .add_systems(
                Update,
                (
                    button_actions,
                    button_interaction::<SettingsButton>,
                    text_cursor,
                )
                    .run_if(in_state(GameState::Settings)),
            )
            .add_systems(OnExit(GameState::Settings), teardown::<OnSettings>);
    }
}

fn c_text_box(a: &mut NodeBundle) {
    a.style.min_height = Val::Px(10.0);
    a.style.min_width = Val::Px(32.0);
    a.background_color = Color::WHITE.into();
}

fn c_text_box_inner(a: &mut NodeBundle) {
    a.style.padding = UiRect::all(Val::Px(8.0));
    a.style.border = UiRect::all(Val::Px(1.0));
    a.border_color = Color::BLACK.into();
    a.background_color = Color::WHITE.into();
}

fn draw_settings(mut commands: Commands, assets: Res<AssetServer>) {
    rooti(
        (c_background, c_centers),
        &assets,
        &mut commands,
        OnSettings,
        |p| {
            node((), p, |p| {
                node((), p, |p| {
                    text("UI Scale", (), (), p);
                    buttoni((), TextBox, p, |p| {
                        node(c_text_box, p, |p| {
                            node(c_text_box_inner, p, |p| {
                                text("2.0", (), c_button_text, p);
                            });
                        });
                    });
                });
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

fn text_cursor(
    query: Query<&Interaction, (Changed<Interaction>, With<TextBox>)>,
    mut window: Query<&mut Window>,
) {
    for interaction in &query {
        for mut window in &mut window {
            match interaction {
                Interaction::Pressed => {}
                Interaction::Hovered => window.cursor.icon = CursorIcon::Text,
                Interaction::None => window.cursor.icon = CursorIcon::Default,
            }
        }
    }
}

#[derive(Component)]
struct TextBox;

#[derive(Component)]
enum SettingsButton {
    Back,
}

#[derive(Component)]
struct OnSettings;
