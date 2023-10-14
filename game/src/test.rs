use bevy::{asset::AssetIo, ui::NodeQuery};

use crate::{
    main_menu::{MainMenuButton, MainMenuPlugin},
    settings_page::SettingsPlugin,
};

use super::*;

struct IO;
impl AssetIo for IO {
    fn load_path<'a>(
        &'a self,
        _path: &'a std::path::Path,
    ) -> bevy::utils::BoxedFuture<'a, Result<Vec<u8>, bevy::asset::AssetIoError>> {
        todo!()
    }

    fn read_directory(
        &self,
        _path: &std::path::Path,
    ) -> Result<Box<dyn Iterator<Item = std::path::PathBuf>>, bevy::asset::AssetIoError> {
        todo!()
    }

    fn get_metadata(
        &self,
        _path: &std::path::Path,
    ) -> Result<bevy::asset::Metadata, bevy::asset::AssetIoError> {
        todo!()
    }

    fn watch_path_for_changes(
        &self,
        _to_watch: &std::path::Path,
        _to_reload: Option<std::path::PathBuf>,
    ) -> Result<(), bevy::asset::AssetIoError> {
        todo!()
    }

    fn watch_for_changes(
        &self,
        _configuration: &bevy::asset::ChangeWatcher,
    ) -> Result<(), bevy::asset::AssetIoError> {
        todo!()
    }
}

#[test]
fn click_on_settings_button_goes_to_settings() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, BasePlugin, MainMenuPlugin, SettingsPlugin));

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
    app.insert_resource(AssetServer::new(IO));

    app.update();
    app.update();
    app.update();

    let state = app.world.get_resource::<State<GameState>>().unwrap().get();

    assert_eq!(state, &GameState::Settings);
}
