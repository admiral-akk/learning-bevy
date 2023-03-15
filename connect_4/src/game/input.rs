use std::marker::PhantomData;

use alpha_beta::{AlphaBetaSearch, GameModel};
use bevy::prelude::*;
use connect_4_model::{types::Player, Model, Move};

use iyes_loopless::prelude::IntoConditionalSystem;
use k_utils::{
    raycast::components::GameInteraction, util_input::handle_input, util_state::StateContraint,
};

use super::{actions::Actions, logic::MoveHistory};

#[derive(Default)]
pub struct Input<StateType: StateContraint> {
    _phantom: PhantomData<StateType>,
}

use k_utils::raycast::components::Interaction::*;
fn propose_move(
    interactions: Query<(&Column, &GameInteraction), Changed<GameInteraction>>,
    move_history: Res<MoveHistory>,
    humans: Query<&Human>,
    mut action_ewr: EventWriter<Actions>,
) {
    let board = Model::from(move_history.0.iter());
    if humans.iter().all(|ap| board.active_player.ne(&ap.0)) {
        return;
    }
    let moves = board.legal_moves();
    for (&pos, &GameInteraction { interaction }) in interactions.iter() {
        if let Some(m) = moves
            .iter()
            .filter(|Move(p, _)| p.x == pos.0 as usize)
            .next()
        {
            match interaction {
                JustReleased => {
                    action_ewr.send(Actions::Move(*m));
                }
                _ => {}
            }
        }
    }
}

impl<StateType: StateContraint> Plugin for Input<StateType> {
    fn build(&self, app: &mut App) {
        handle_input::<StateType>(app, vec![propose_move.into_conditional()]);
    }
}

// Input
#[derive(Component, PartialEq, Clone, Copy, Debug)]
pub struct Human(pub Player);

#[derive(Component)]
pub struct Bot(pub Player, pub Option<AlphaBetaSearch<Move>>);

#[derive(Component, PartialEq, Clone, Copy, Hash, Eq, Debug)]
pub struct Column(pub i32);
