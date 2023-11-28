use std::net::UdpSocket;
use std::time::Duration;

use base::{MoveEvent, NetId};
use bevy::prelude::*;
use bevy_renet::renet::transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use bevy_renet::transport::NetcodeServerPlugin;
use bevy_renet::RenetServerPlugin;

pub struct ServerPlugin;
impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenetServerPlugin);
        app.add_plugins(NetcodeServerPlugin);

        let public_addr = "127.0.0.1:5000".parse().unwrap();
        let socket = UdpSocket::bind(public_addr).unwrap();

        let server_config = ServerConfig {
            current_time: Duration::default(),
            max_clients: 64,
            protocol_id: 1,
            public_addresses: vec![public_addr],
            authentication: ServerAuthentication::Unsecure,
        };

        let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
        let server = RenetServer::new(ConnectionConfig::default());
        app.insert_resource(transport);
        app.insert_resource(server);

        app.add_systems(Update, (read_move_events, read_server_events));
        app.add_systems(PostUpdate, send_move_updates);
    }
}

fn read_move_events(mut server: ResMut<RenetServer>, mut events: EventWriter<MoveEvent>) {
    for client_id in server.clients_id() {
        let message = server.receive_message(client_id, DefaultChannel::ReliableOrdered);
        if let Some(message) = message {
            let message: MoveEvent = bincode::deserialize(&message).unwrap();
            events.send(message);
        }
    }
}

fn read_server_events(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Player {} connected.", client_id);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Player {} disconnected: {}", client_id, reason);
            }
        }
    }
}

fn send_move_updates(
    moves: Query<(&Transform, &NetId), Changed<Transform>>,
    mut server: ResMut<RenetServer>,
) {
    for m in &moves {
        println!("sending update");
        let message = MoveEvent {
            pos: m.0.translation,
            entity: *m.1,
        };
        let message = bincode::serialize(&message).unwrap();
        server.broadcast_message(DefaultChannel::ReliableOrdered, message);
    }
}
