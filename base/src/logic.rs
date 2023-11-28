use bevy::prelude::*;

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

fn is_host(play: Res<State<PlayType>>) -> bool {
    match play.as_ref().get() {
        PlayType::None => false,
        PlayType::Single => true,
        PlayType::Multi => false,
        PlayType::Server => true,
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
