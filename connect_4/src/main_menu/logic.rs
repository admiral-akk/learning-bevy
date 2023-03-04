use std::marker::PhantomData;

use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::{
    game::plugin::Game,
    utils::{
        util_action::{handle_actions, Proposal, Selection},
        util_plugin::UtilPlugin,
        util_state::StateContraint,
    },
};

use super::{
    event::{StartGame, StartingGame},
    plugin::MainMenu,
};

fn simulate_moves(mut commands: Commands, start_game: Query<Entity, (&StartGame, With<Proposal>)>) {
    for entity in start_game.iter() {
        commands.get_entity(entity).unwrap().insert(StartingGame);
    }
}

fn apply_move(mut commands: Commands, actions: Query<&StartGame, With<Selection>>) {
    for _ in actions.iter() {
        MainMenu::exit_to::<Game>(&mut commands);
        return;
    }
}

#[derive(Default)]
pub struct Logic<StateType: StateContraint> {
    _phantom: PhantomData<StateType>,
}

impl<StateType: StateContraint> Plugin for Logic<StateType> {
    fn build(&self, app: &mut App) {
        handle_actions::<StateType>(
            app,
            simulate_moves.into_conditional(),
            apply_move.into_conditional(),
        );
    }
}
