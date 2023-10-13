use bevy::{app::AppExit, prelude::*};
use bevy_ui_dsl::*;

use crate::GameState;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), draw_main_menu)
            .add_systems(
                Update,
                (button_interaction, button_click).run_if(in_state(GameState::MainMenu)),
            );
    }
}

fn c_centers(node: &mut NodeBundle) {
    node.style.display = Display::Flex;
    node.style.align_content = AlignContent::SpaceAround;
    node.style.justify_content = JustifyContent::SpaceAround;
    node.style.align_items = AlignItems::Center;
    node.style.justify_items = JustifyItems::Center;
}

fn c_background(node: &mut NodeBundle) {
    node.background_color = Color::DARK_GREEN.into();
    node.style.height = Val::Percent(100.0);
    node.style.width = Val::Percent(100.0);
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

fn c_button(_: &AssetServer, node: &mut ButtonBundle) {
    node.style.padding = UiRect::all(Val::Px(8.0));
}

fn c_button_text(_b: &AssetServer, a: &mut TextStyle) {
    a.color = Color::BLACK;
}

fn c_title_test(_: &AssetServer, a: &mut TextStyle) {
    a.font_size = 32.0;
}

#[derive(Component)]
enum MyButton {
    SinglePlayer,
    MultiPlayer,
    Settings,
    Quit,
}

fn draw_main_menu(mut commands: Commands, assets: Res<AssetServer>) {
    root((c_background, c_centers), &assets, &mut commands, |p| {
        node(c_main_menu, p, |p| {
            text("The Tales our\nAncestors Told", (), c_title_test, p);

            buttoni(c_button, MyButton::SinglePlayer, p, |p| {
                text("Single Player", (), c_button_text, p);
            });
            buttoni(c_button, MyButton::MultiPlayer, p, |p| {
                text("Multi Player", (), c_button_text, p);
            });
            buttoni(c_button, MyButton::Settings, p, |p| {
                text("Settings", (), c_button_text, p);
            });
            buttoni(c_button, MyButton::Quit, p, |p| {
                text("Quit", (), c_button_text, p);
            });
        });
    });
}

fn button_interaction(
    query: Query<&Interaction, (Changed<Interaction>, With<MyButton>)>,
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

fn button_click(
    query: Query<(&Interaction, &MyButton), Changed<Interaction>>,
    mut exit_event: ResMut<Events<AppExit>>,
) {
    for (interaction, button) in &query {
        if let MyButton::Quit = button {
            if let Interaction::Pressed = interaction {
                exit_event.send(AppExit)
            }
        }
    }
}
