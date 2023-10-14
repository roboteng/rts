use bevy::{app::AppExit, prelude::*};
use bevy_ui_dsl::*;

use crate::{classes::*, *};

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), draw_main_menu)
            .add_systems(
                Update,
                (button_interaction::<MainMenuButton>, button_click)
                    .run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), teardown::<OnMenuScreen>);
    }
}

fn c_main_menu(node: &mut NodeBundle) {
    node.background_color = Color::MIDNIGHT_BLUE.into();
    node.style.padding = UiRect::all(Val::Px(20.0));
    node.style.flex_direction = FlexDirection::Column;
    node.style.align_items = AlignItems::Center;
    node.style.justify_content = JustifyContent::Start;
    node.style.row_gap = Val::Px(10.0);

    node.style.max_width = Val::Vw(33.3);
}

fn c_title_test(_: &AssetServer, a: &mut TextStyle) {
    a.font_size = 32.0;
}

#[derive(Component)]
struct OnMenuScreen;

#[derive(Component)]
enum MainMenuButton {
    SinglePlayer,
    MultiPlayer,
    Settings,
    Quit,
}

fn draw_main_menu(mut commands: Commands, assets: Res<AssetServer>) {
    rooti(
        (c_background, c_centers),
        &assets,
        &mut commands,
        OnMenuScreen,
        |p| {
            node(c_main_menu, p, |p| {
                text("The Tales our\nAncestors Told", (), c_title_test, p);

                buttoni(c_button, MainMenuButton::SinglePlayer, p, |p| {
                    text("Single Player", (), c_button_text, p);
                });
                buttoni(c_button, MainMenuButton::MultiPlayer, p, |p| {
                    text("Multi Player", (), c_button_text, p);
                });
                buttoni(c_button, MainMenuButton::Settings, p, |p| {
                    text("Settings", (), c_button_text, p);
                });
                buttoni(c_button, MainMenuButton::Quit, p, |p| {
                    text("Quit", (), c_button_text, p);
                });
            });
        },
    );
}

fn button_click(
    query: Query<(&Interaction, &MainMenuButton), Changed<Interaction>>,
    mut exit_event: ResMut<Events<AppExit>>,
    mut states: ResMut<NextState<GameState>>,
) {
    for (interaction, button) in &query {
        if let Interaction::Pressed = interaction {
            match button {
                MainMenuButton::Quit => exit_event.send(AppExit),
                MainMenuButton::Settings => {
                    states.set(GameState::Settings);
                }
                _ => {}
            }
        }
    }
}
