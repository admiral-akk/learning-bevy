use std::marker::PhantomData;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use k_utils::{util_action::Proposal, util_input::handle_input, util_state::StateContraint};

use super::{event::StartGame, plugin::MainMenu};
fn propose_moves(
    mut commands: Commands,
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<MainMenu>),
    >,
) {
    for &interaction in &mut interaction_query {
        match interaction {
            Interaction::Hovered => {
                Proposal::propose_action(&mut commands, false, 0, StartGame);
            }
            Interaction::Clicked => {
                Proposal::propose_action(&mut commands, true, 1, StartGame);
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
        handle_input::<StateType>(app, vec![propose_moves.into_conditional()]);
    }
}
