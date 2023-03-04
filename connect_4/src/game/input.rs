use std::marker::PhantomData;

use alpha_beta::{AlphaBetaSearch, GameModel};
use bevy::prelude::*;
use connect_4_model::{types::Player, Model, Move};
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::utils::{
    raycast::components::{GameInteraction, Interaction::*},
    util_action::Proposal,
    util_input::handle_input,
    util_state::StateContraint,
};

use super::{
    event::{ExitGame, MoveW},
    logic::MoveHistory,
    plugin::Game,
};
fn propose_moves(
    mut commands: Commands,
    interactions: Query<(&Column, &GameInteraction), Changed<GameInteraction>>,
    move_history: Res<MoveHistory>,
    humans: Query<&Human>,
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
                Hover | JustClicked | Clicked => {
                    Proposal::propose_action(&mut commands, false, 1, MoveW::new(*m));
                }
                JustReleased => {
                    Proposal::propose_action(&mut commands, true, 1, MoveW::new(*m));
                }
                _ => {}
            }
        }
    }
}

fn propose_exit(
    mut commands: Commands,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<Game>)>,
) {
    for &interaction in interaction_query.iter() {
        match interaction {
            Interaction::Clicked => {
                Proposal::propose_action(&mut commands, true, 0, ExitGame);
            }
            Interaction::Hovered => {
                Proposal::propose_action(&mut commands, false, 0, ExitGame);
            }
            _ => {}
        }
    }
}

#[derive(Default)]
pub struct Input<StateType: StateContraint> {
    _phantom: PhantomData<StateType>,
}

impl<StateType: StateContraint> Plugin for Input<StateType> {
    fn build(&self, app: &mut App) {
        handle_input::<StateType>(
            app,
            vec![
                propose_moves.into_conditional(),
                propose_exit.into_conditional(),
            ],
        );
    }
}

// Input

#[derive(Component, PartialEq, Clone, Copy, Debug)]
pub struct Human(pub Player);

#[derive(Component)]
pub struct Bot(pub Player, pub Option<AlphaBetaSearch<Move>>);

#[derive(Component, PartialEq, Clone, Copy, Hash, Eq, Debug)]
pub struct Column(pub i32);
