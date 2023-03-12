use std::marker::PhantomData;

use crate::game::plugin::Game;
use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;
use k_utils::{
    util_action::{handle_actions, Act},
    util_plugin::UtilPlugin,
    util_state::StateContraint,
};

use super::{actions::Actions, plugin::MainMenu};

fn apply_move(mut commands: Commands, mut action_ewr: EventReader<Act<Actions>>) {
    for action in action_ewr.iter() {
        match action.action {
            Actions::StartGame(s) => match s {
                k_utils::util_button::State::JustReleased(_) => {
                    println!("Just released event!");
                    MainMenu::exit_to::<Game>(&mut commands);
                    return;
                }
                _ => {}
            },
        }
    }
}

#[derive(Default)]
pub struct Logic<StateType: StateContraint> {
    _phantom: PhantomData<StateType>,
}

impl<StateType: StateContraint> Plugin for Logic<StateType> {
    fn build(&self, app: &mut App) {
        handle_actions::<StateType>(app, apply_move.into_conditional());
    }
}
