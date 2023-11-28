use std::{net::UdpSocket, time::Duration};

use bevy::prelude::*;
use bevy_renet::{
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport},
        ConnectionConfig, DefaultChannel, RenetClient,
    },
    transport::NetcodeClientPlugin,
    RenetClientPlugin,
};

use crate::{setup, GameState, MoveEvent, NetId, PlayType, UserCommands};

pub struct GameLogicPlugin;
impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup);
        app.add_systems(
            Update,
            (process_user_commands, process_commands).run_if(is_host),
        );
    }
}

pub struct MultiplayerClientPlugin;
impl Plugin for MultiplayerClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenetClientPlugin);
        app.add_plugins(NetcodeClientPlugin);

        app.insert_resource(RenetClient::new(ConnectionConfig::default()));
        app.insert_resource(
            NetcodeClientTransport::new(
                Duration::default(),
                ClientAuthentication::Unsecure {
                    protocol_id: 1,
                    client_id: 1,
                    server_addr: "127.0.0.1:5000".parse().unwrap(),
                    user_data: None,
                },
                UdpSocket::bind("127.0.0.1:0").unwrap(),
            )
            .unwrap(),
        );

        app.add_systems(
            Update,
            (send_commands, take_server_messages).run_if(is_client),
        );
    }
}

fn take_server_messages(mut q: Query<(&mut Transform, &NetId)>, mut client: ResMut<RenetClient>) {
    if let Some(k) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let message: MoveEvent = bincode::deserialize(&k).unwrap();
        let mut k = q.iter_mut().find(|m| m.1 == &message.entity).unwrap();
        k.0.translation = message.pos;
    }
}

fn is_host(play: Res<State<PlayType>>) -> bool {
    match play.as_ref().get() {
        PlayType::None => false,
        PlayType::Single => true,
        PlayType::Multi => false,
        PlayType::Server => true,
    }
}

fn is_client(play: Res<State<PlayType>>) -> bool {
    match play.as_ref().get() {
        PlayType::None => false,
        PlayType::Single => false,
        PlayType::Multi => true,
        PlayType::Server => false,
    }
}

fn send_commands(mut selections: EventReader<MoveEvent>, mut client: ResMut<RenetClient>) {
    for event in selections.read() {
        let _ = bincode::serialize(event)
            .map(|message| client.send_message(DefaultChannel::ReliableOrdered, message));
    }
}

fn process_commands(
    mut selections: EventReader<MoveEvent>,
    mut selecteds: Query<(&mut UserCommands, &NetId)>,
) {
    for event in selections.read() {
        for (mut selection, id) in selecteds.iter_mut() {
            if event.entity == *id {
                selection.0.push(event.pos);
            }
        }
    }
}

fn process_user_commands(mut actions: Query<(&mut Transform, &mut UserCommands)>) {
    for (mut trans, mut comms) in actions.iter_mut() {
        if comms.0.is_empty() {
            continue;
        }
        trans.translation = comms.0[0];
        comms.0.clear();
    }
}
