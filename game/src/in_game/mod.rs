use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_ui_dsl::*;

use base::{logic::GameLogicPlugin, *};

pub mod multiplayer;

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPickingPlugins, GameLogicPlugin, ClientGUIPlugin))
            .add_event::<MoveEvent>();
    }
}

fn is_client(game: Res<State<GameState>>, play: Res<State<PlayType>>) -> bool {
    let k = (*game.as_ref().get(), *play.as_ref().get());

    matches!(
        k,
        (GameState::InGame, PlayType::Single) | (GameState::InGame, PlayType::Multi)
    )
}

struct ClientGUIPlugin;
impl Plugin for ClientGUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), draw_hud.run_if(is_client));

        app.add_systems(
            Update,
            (show_selection, generate_commands).run_if(is_client),
        );
    }
}

fn c_hud(n: &mut NodeBundle) {
    n.style.width = Val::Vw(100.0);
    n.style.height = Val::Vh(20.0);
    n.style.top = Val::Vh(80.0);
    n.style.position_type = PositionType::Absolute;
    n.background_color = Color::MAROON.into();
}

fn draw_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    rooti(
        c_hud,
        &asset_server,
        &mut commands,
        (
            Hud,
            Pickable {
                should_block_lower: true,
                should_emit_events: false,
            },
        ),
        |p| {
            texti("", (), (), HealthVis, p);
        },
    );
}

#[derive(Component)]
struct Hud;

fn show_selection(
    q: Query<(&Health, &PickSelection)>,
    mut texts: Query<&mut Text, With<HealthVis>>,
) {
    let selections = q
        .iter()
        .filter(|(_, s)| s.is_selected)
        .map(|(h, _)| h)
        .collect::<Vec<_>>();
    for mut text in &mut texts {
        if selections.len() == 1 {
            let health = selections[0];
            text.sections = vec![TextSection {
                value: format!("{} of {}", health.current, health.max),
                style: TextStyle::default(),
            }];
        } else {
            text.sections = vec![];
        }
    }
}

fn generate_commands(
    grounds: Query<Entity, With<Ground>>,
    mut ev: EventReader<Pointer<Click>>,
    mut selections: EventWriter<MoveEvent>,
    selecteds: Query<(&PickSelection, &NetId), With<UserCommands>>,
) {
    if let Some(ground) = grounds.iter().next() {
        for e in ev.read() {
            if e.target == ground && PointerButton::Secondary == e.button {
                for (selection, &id) in &selecteds {
                    if !selection.is_selected {
                        continue;
                    }
                    if let Some(pos) = e.hit.position {
                        let event = MoveEvent { pos, entity: id };
                        selections.send(event);
                    }
                }
            }
        }
    }
}
