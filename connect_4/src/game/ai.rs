use std::{marker::PhantomData, time::Duration};

use alpha_beta::{AlphaBetaSearch, GameModel};
use bevy::prelude::*;
use connect_4_model::{Model, Move};
use iyes_loopless::prelude::IntoConditionalSystem;

use super::{input::Bot, logic::MoveHistory};
use crate::game::event::MoveW;
use k_utils::{util_action::Proposal, util_input::handle_input, util_state::StateContraint};

fn select_move(board: &mut Model, search: &mut Option<AlphaBetaSearch<Move>>) -> Option<Move> {
    let depth = 6;
    let m = board.search(depth);
    match search {
        Some(search) => search.continue_search(board, Duration::from_millis(2000)),
        None => {
            *search = Some(AlphaBetaSearch::start_search(
                board,
                depth,
                Duration::from_millis(20),
            ));
        }
    }
    match search {
        Some(unwrapped_search) => {
            if unwrapped_search.complete {
                match &unwrapped_search.best_so_far {
                    Some(result) => {
                        if !m.unwrap().eq(&result.m) {
                            println!("MOVE DISAGREE!");
                            println!("Good move: {:?}", m);
                            println!("Move: {:?}, score: {}", result.m, result.score);
                        }
                        let m = result.m;
                        println!("Move: {:?}, score: {}", m, result.score);
                        *search = None;
                        Some(m)
                    }
                    None => None,
                }
            } else {
                None
            }
        }
        None => {
            println!("PANIC!");
            None
        }
    }
}

fn propose_moves(
    mut commands: Commands,
    move_history: Res<MoveHistory>,
    mut bots: Query<&mut Bot>,
) {
    let mut board = Model::from(move_history.0.iter());
    if bots.iter().all(|b| board.active_player.ne(&b.0)) {
        return;
    }

    let mut bot = bots
        .iter_mut()
        .filter(|b| board.active_player.eq(&b.0))
        .last()
        .unwrap();

    match select_move(&mut board, &mut bot.1) {
        Some(m) => {
            println!("Move: {:?}", m);
            Proposal::propose_action(&mut commands, true, 1, MoveW::new(m));
        }
        None => {}
    }
}

#[derive(Default)]
pub struct AI<StateType: StateContraint> {
    _phantom: PhantomData<StateType>,
}

impl<StateType: StateContraint> Plugin for AI<StateType> {
    fn build(&self, app: &mut App) {
        handle_input::<StateType>(app, vec![propose_moves.into_conditional()]);
    }
}
