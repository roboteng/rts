use bevy::{app::AppExit, prelude::*};
use bevy_ui_dsl::*;

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GameState>()
            .add_systems(Startup, draw_main_menu)
            .add_systems(Update, button_interaction);
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
    a.color = Color::BLACK.into();
}

fn c_title_test(_: &AssetServer, a: &mut TextStyle) {
    a.font_size = 32.0;
}

#[derive(Component)]
struct QuitButton;

fn draw_main_menu(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    root((c_background, c_centers), &assets, &mut commands, |p| {
        node(c_main_menu, p, |p| {
            text("The Tales our\nAncestors Told", (), c_title_test, p);

            button(c_button, p, |p| {
                text("Single Player", (), c_button_text, p);
            });
            button(c_button, p, |p| {
                text("Multi Player", (), c_button_text, p);
            });
            button(c_button, p, |p| {
                text("Settings", (), c_button_text, p);
            });
            buttoni(c_button, QuitButton, p, |p| {
                text("Quit", (), c_button_text, p);
            });
        });
    });
}

fn button_interaction(
    query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut window: Query<&mut Window>,
    mut exit_event: ResMut<Events<AppExit>>,
) {
    for interaction in &query {
        for mut window in &mut window {
            window.cursor.icon = match interaction {
                Interaction::Pressed => {
                    exit_event.send(AppExit);
                    CursorIcon::Default
                }
                Interaction::Hovered => CursorIcon::Hand,
                Interaction::None => CursorIcon::Default,
            }
        }
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, States)]
enum GameState {
    #[default]
    MainMenu,
    Settings,
}
