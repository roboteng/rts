use bevy::prelude::*;

pub mod classes;
pub mod main_menu;

pub struct BasePlugin;
impl Plugin for BasePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GameState>().add_systems(Startup, setup_2d);
    }
}

fn setup_2d(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, States)]
enum GameState {
    #[default]
    MainMenu,
    Settings,
}

pub mod settings_page {
    use bevy::prelude::*;
    use bevy_ui_dsl::*;

    use crate::{classes::*, GameState};

    pub struct SettingsPlugin;
    impl Plugin for SettingsPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::Settings), draw_settings)
                .add_systems(Update, button_actions.run_if(in_state(GameState::Settings)))
                .add_systems(OnExit(GameState::Settings), teardown);
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
        for i in &query {
            if i.0 == &Interaction::Pressed {
                match i.1 {
                    SettingsButton::Back => state.set(GameState::MainMenu),
                }
            }
        }
    }

    fn teardown(mut commands: Commands, query: Query<Entity, With<OnSettings>>) {
        for e in &query {
            commands.entity(e).despawn_recursive();
        }
    }

    #[derive(Component)]
    enum SettingsButton {
        Back,
    }

    #[derive(Component)]
    struct OnSettings;
}
