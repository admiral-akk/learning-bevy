use bevy::prelude::*;

use k_utils::{util_plugin::UtilPlugin, util_state::StateContraint};

use super::{actions::Actions, graphics::Graphics, logic::Logic, ui::*};

#[derive(Debug, Default, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        Self::add_defaults(app);
        app.add_plugin(UI::default());
        app.add_plugin(Logic::default());
        app.add_plugin(Graphics::default());
    }
}

impl StateContraint for Game {}
impl UtilPlugin<Game, Actions> for Game {}
