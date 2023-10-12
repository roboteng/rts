use bevy::prelude::*;

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GameState>()
            .add_systems(Startup, draw_main_menu);
    }
}

fn draw_main_menu(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                width: Val::Vw(100.0),
                ..Default::default()
            },
            background_color: Color::DARK_GREEN.into(),
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "Hello, World!",
                TextStyle::default(),
            ));
            builder
                .spawn(ButtonBundle {
                    ..Default::default()
                })
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Hello, Button!",
                        TextStyle {
                            color: Color::BLACK,
                            ..Default::default()
                        },
                    ));
                });
        });
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, States)]
enum GameState {
    #[default]
    MainMenu,
    Settings,
}
