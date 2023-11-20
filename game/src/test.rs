use bevy::ui::NodeQuery;

use crate::{
    main_menu::{MainMenuButton, MainMenuPlugin},
    settings_page::SettingsPlugin,
};

use super::*;

#[ignore = "broken after upgrading bevy"]
#[test]
fn click_on_settings_button_goes_to_settings() {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        BasePlugin::default(),
        MainMenuPlugin,
        SettingsPlugin,
    ));

    fn click_settings(
        mut commands: Commands,
        query: Query<(Entity, &MainMenuButton)>,
        mut node_query: Query<NodeQuery>,
    ) {
        for (e, b) in &query {
            match b {
                MainMenuButton::Settings => {
                    commands.entity(e);
                    if let Ok(mut k) = node_query.get_component_mut::<Interaction>(e) {
                        *k = Interaction::Pressed;
                    }
                }
                _ => (),
            }
        }
    }

    app.add_systems(Update, click_settings);
    // app.insert_resource(AssetServer::new(IO));
    app.insert_resource(UiScale(1.0));

    app.update();
    app.update();
    app.update();

    let state = app.world.get_resource::<State<GameState>>().unwrap().get();

    assert_eq!(state, &GameState::Settings);
}
