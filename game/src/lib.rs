use bevy::prelude::*;
use bevy_ui_dsl::*;

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GameState>()
            .add_systems(Startup, draw_main_menu);
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

fn c_m(node: &mut NodeBundle) {
    node.background_color = Color::MIDNIGHT_BLUE.into();
    node.style.padding = UiRect::all(Val::Px(20.0));
    node.style.flex_direction = FlexDirection::Column;
    node.style.align_items = AlignItems::Center;
    node.style.justify_content = JustifyContent::Start;
    node.style.column_gap = Val::Px(10.0);
}

fn c_child(node: &mut NodeBundle) {
    node.background_color = Color::ORANGE_RED.into();
}

fn c_button_text(_b: &AssetServer, a: &mut TextStyle) {
    a.color = Color::BLACK.into();
}

fn draw_main_menu(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    root((c_background, c_centers), &assets, &mut commands, |p| {
        node(c_m, p, |p| {
            text("Hello, DSL", (), (), p);
            node(c_child, p, |p| {
                text("Goodbye, DSL", (), (), p);
            });
            button((), p, |p| {
                text("Hello, button", (), c_button_text, p);
            });
        });
    });
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, States)]
enum GameState {
    #[default]
    MainMenu,
    Settings,
}
