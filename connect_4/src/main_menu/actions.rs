use bevy::prelude::{Commands, EventReader};
use iyes_loopless::{condition::ConditionalSystemDescriptor, prelude::IntoConditionalSystem};
use k_utils::{util_action::Action, util_button::State, util_plugin::UtilPlugin};

use crate::{game::plugin::Game, main_menu::plugin::MainMenu};

#[derive(Clone, Copy)]
pub enum Actions {
    StartGame(State),
}

impl Action for Actions {
    fn apply_move() -> ConditionalSystemDescriptor {
        apply_move.into_conditional()
    }
}

fn apply_move(mut commands: Commands, mut action_ewr: EventReader<Actions>) {
    for action in action_ewr.iter() {
        match action {
            Actions::StartGame(s) => match s {
                k_utils::util_button::State::JustReleased(_) => {
                    MainMenu::exit_to::<Game>(&mut commands);
                    return;
                }
                _ => {}
            },
        }
    }
}
