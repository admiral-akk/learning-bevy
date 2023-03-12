use bevy::prelude::*;

use k_utils::{util_plugin::UtilPlugin, util_state::StateContraint};

use super::{actions::Actions, ai::AI, graphics::Graphics, input::Input, logic::Logic, ui::UI};

#[derive(Debug, Default, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Game;

impl UtilPlugin<Game, Actions> for Game {}

impl StateContraint for Game {}

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        Self::add_defaults(app);
        app.add_plugin(Logic::<Game>::default());
        app.add_plugin(Graphics::<Game>::default());
        app.add_plugin(Input::<Game>::default());
        app.add_plugin(UI::<Game>::default());
        app.add_plugin(AI::<Game>::default());
    }
}
