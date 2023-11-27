use bevy::prelude::*;
use bevy_renet::transport::NetcodeServerPlugin;
use bevy_renet::RenetServerPlugin;

pub struct ServerPlugin;
impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenetServerPlugin);
        app.add_plugins(NetcodeServerPlugin);
    }
}
