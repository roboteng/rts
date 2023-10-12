use bevy::prelude::*;
use bevy_ui_dsl::*;

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GameState>()
            .add_systems(Startup, draw_main_menu);
    }
}

fn c_background(node: &mut NodeBundle) {
    node.background_color = Color::DARK_GREEN.into();
    let style = &mut node.style;
    style.flex_direction = FlexDirection::Column;
    style.width = Val::Vw(100.0);
}

fn draw_main_menu(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    root(c_background, &assets, &mut commands, |p| {
        text("Hello, DSL", (), (), p);
    });
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, States)]
enum GameState {
    #[default]
    MainMenu,
    Settings,
}
