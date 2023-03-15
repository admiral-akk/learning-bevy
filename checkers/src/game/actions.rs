use bevy::prelude::{Commands, EventReader};
use iyes_loopless::{condition::ConditionalSystemDescriptor, prelude::IntoConditionalSystem};
use k_utils::{util_action::Action, util_button::State, util_plugin::UtilPlugin};

use crate::start_menu::plugin::StartMenu;

use super::plugin::Game;

#[derive(Clone, Copy)]
pub enum Actions {
    ExitGame(State),
}

impl Action for Actions {
    fn apply_move() -> ConditionalSystemDescriptor {
        apply_move.into_conditional()
    }
}

fn apply_move(mut commands: Commands, mut action_ewr: EventReader<Actions>) {
    for action in action_ewr.iter() {
        match action {
            Actions::ExitGame(s) => match s {
                k_utils::util_button::State::JustReleased(_) => {
                    Game::exit_to::<StartMenu>(&mut commands);
                    return;
                }
                _ => {}
            },
        }
    }
}
