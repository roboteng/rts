use bevy::prelude::*;

struct BasePlugin;
impl Plugin for BasePlugin {
    fn build(&self, _app: &mut App) {
        //app.;
    }
}

struct CoreLogicPlugin;
impl Plugin for CoreLogicPlugin {
    fn build(&self, app: &mut App) {
        // app.;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn move_a_villager() {
        let mut app = App::new();

        app.add_plugins((MinimalPlugins, BasePlugin, CoreLogicPlugin));
    }
}
